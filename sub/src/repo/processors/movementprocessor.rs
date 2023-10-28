use crate::data::threads::movechannel::{MovementChannelData, MovementChannelDataType};
use crate::repo::physical::motors::base::Motor;
use std::time::Instant;

pub struct MovementProcessor<T: Motor> {
    last_set: Instant,
    motors: T,
}

impl<T: Motor> MovementProcessor<T> {
    pub fn new(motors: T) -> Self {
        MovementProcessor {
            last_set: Instant::now(),
            motors,
        }
    }

    pub fn process_message(&mut self, message: MovementChannelData) {
        if message.data_type == MovementChannelDataType::Move {
            if message.data.is_none() {
                return;
            }

            self.last_set = Instant::now();

            let data = message.data.unwrap();
            self.motors
                .set_acceleration(data.movement_command[0].clone());
            self.motors.set_pitch(data.movement_command[1].clone());
            self.motors.set_yaw(data.movement_command[2].clone());
            self.motors.set_roll(data.movement_command[3].clone());
        }

        if message.data_type == MovementChannelDataType::CheckTimeout {
            if self.last_set.elapsed().as_secs() > 5 {
                self.motors.disengage();
            }
        }
    }
}
