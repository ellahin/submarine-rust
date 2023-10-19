mod data;
mod repo;

use crate::data::threads::movechannel::{MovementChannelData, MovementChannelDataType};
use crate::repo::physical::motors::generic::MotorGeneric;
use crate::repo::processors::movementprocessor::MovementProcessor;
use common_data::commands::movement::Movement;
use opencv::{
    core::{Mat, Vector},
    imgcodecs,
    prelude::VideoCaptureTrait,
    videoio,
};
use repo::physical::motors::base::Motor;
use std::io::Write;
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
                let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)
                    .expect("Failed to get video capture");
                let mut frame = Mat::default();
                let mut buf = Vector::new();
                cam.read(&mut frame).expect("Failed to capture frame");
                buf.clear();
                let _ = imgcodecs::imencode(".jpg", &frame, &mut buf, &Vector::new());

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: multipart/x-mixed-replace; boundary=frame\r\n\r\n"
                );

                stream.write_all(response.as_bytes()).unwrap();

                loop {
                    cam.read(&mut frame).expect("Failed to capture frame");
                    buf.clear();
                    let _ = imgcodecs::imencode(".jpg", &frame, &mut buf, &Vector::new());

                    let image_data = format!(
                        "--frame\r\nContent-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                        buf.len()
                    );

                    stream.write_all(image_data.as_bytes()).unwrap();
                    stream.write_all(buf.as_slice()).unwrap();
                    stream.write_all(b"\r\n").unwrap();
                    stream.flush().unwrap();
                }
            });
        }
    });

    let _ = ping_thread.join();
}
