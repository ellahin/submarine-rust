mod data;
mod repo;

use crate::data::threads::movechannel::{MovementChannelData, MovementChannelDataType};
use crate::repo::physical::motors::servosixfront::ServoSixFront;
use crate::repo::processors::movementprocessor::MovementProcessor;
use crate::repo::processors::stream::StreamHandler;
use repo::physical::motors::base::Motor;
use rscam::{Camera, Config};
use std::io::Write;
use std::net::TcpListener;
use std::sync::mpsc;
use std::{thread, time};
fn main() {
    let (movement_tx, movement_rx) = mpsc::channel();

    //Movement Thread
    thread::spawn(move || {
        let motors = ServoSixFront::new();

        let mut movement = MovementProcessor::new(motors);

        for channel_message in movement_rx.iter() {
            movement.process_message(channel_message);
        }
    });

    // Movement Ping thread
    let ping_movement_tx = movement_tx.clone();

    let ping_thread = thread::spawn(move || loop {
        ping_movement_tx
            .send(MovementChannelData {
                data_type: MovementChannelDataType::CheckTimeout,
                data: None,
            })
            .unwrap();
        thread::sleep(time::Duration::from_secs(1));
    });

    // Video streaming Thread
    thread::spawn(|| {
        let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
        println!("Video stram listening on port 8080");

        for stream_incoming in listener.incoming() {
            let mut stream = stream_incoming.unwrap();

            thread::spawn(move || {
                let mut camera = Camera::new("/dev/video0").unwrap();

                camera
                    .start(&Config {
                        interval: (1, 30),
                        resolution: (640, 480),
                        format: b"MJPG",
                        ..Default::default()
                    })
                    .unwrap();

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\n\r\n"
                );

                stream.write_all(response.as_bytes()).unwrap();

                loop {
                    let buff = camera.capture().unwrap();

                    let image_data = format!(
                        "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                        buff.len()
                    );

                    stream.write_all(image_data.as_bytes()).unwrap();
                    stream.write_all(&buff).unwrap();
                    stream.write_all(b"\r\n").unwrap();
                    stream.flush().unwrap();
                }
            });
        }
    });

    // TCP command handler
    let movement_listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    for stream in movement_listener.incoming() {
        let stream_movement_chanel = movement_tx.clone();
        thread::spawn(move || {
            let handler = StreamHandler::new();
            handler.handle(stream.unwrap(), stream_movement_chanel);
        });
    }
    let _ = ping_thread.join();
}
