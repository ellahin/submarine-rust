mod data;
mod repo;

use crate::data::threads::movechannel::{MovementChannelData, MovementChannelDataType};
use crate::repo::physical::motors::generic::MotorGeneric;
use crate::repo::processors::movementprocessor::MovementProcessor;
use crate::repo::processors::stream::StreamHandler;
use repo::physical::motors::base::Motor;
use rscam::{new, Camera, Config};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::{thread, time};
fn main() {
    let (movement_tx, movement_rx) = mpsc::channel();

    //Movement Thread
    thread::spawn(move || {
        let motors = MotorGeneric::new();

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

                let mut buff = camera.capture().unwrap();

                stream.write_all(response.as_bytes()).unwrap();

                loop {
                    buff = camera.capture().unwrap();

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
        let handler = StreamHandler::new();
        let movement_chanel = &movement_tx.clone();
        handler.handle(stream.unwrap(), movement_chanel.clone())
    }
    let _ = ping_thread.join();
}
