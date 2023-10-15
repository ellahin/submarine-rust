mod data;
mod repo;

use crate::data::threads::movechannel::{MovementChannelData, MovementChannelDataType};
use crate::repo::physical::motors::generic::MotorGeneric;
use crate::repo::processors::movementprocessor::MovementProcessor;
use common_data::commands::movement::Movement;
use repo::physical::motors::base::Motor;
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

    let _ = ping_thread.join();
}
