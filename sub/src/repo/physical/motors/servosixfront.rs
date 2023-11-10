use crate::repo::physical::motors::base::{Motor, MotorState};
use common_data::libs::math::maprang::map_range;

use rppal::gpio::{Gpio, OutputPin};
use std::thread::sleep;
use std::time::Duration;
const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 1000;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 2000;

fn default_map(position: i8) -> u64 {
    return map_range(position, -100, 100, PULSE_MIN_US, PULSE_MAX_US);
}

pub struct ServoSixFront {
    motor_front_left: OutputPin,
    motor_front_right: OutputPin,
    motor_back_left: OutputPin,
    motor_back_right: OutputPin,
    motor_front_turn: OutputPin,
    motor_back_thrust: OutputPin,
    motor_state: MotorState,
}

impl Motor for ServoSixFront {
    fn new() -> Self {
        let mut return_struct = ServoSixFront {
            motor_front_left: Gpio::new().unwrap().get(7).unwrap().into_output(),
            motor_front_right: Gpio::new().unwrap().get(11).unwrap().into_output(),
            motor_back_left: Gpio::new().unwrap().get(13).unwrap().into_output(),
            motor_back_right: Gpio::new().unwrap().get(15).unwrap().into_output(),
            motor_front_turn: Gpio::new().unwrap().get(29).unwrap().into_output(),
            motor_back_thrust: Gpio::new().unwrap().get(31).unwrap().into_output(),
            motor_state: MotorState::Disarmed,
        };

        return return_struct;
    }

    fn set_pitch(&mut self, motion: i8) {
        if self.motor_state == MotorState::Disarmed {
            return;
        }

        if motion == 0 {
            self.motor_front_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(PULSE_NEUTRAL_US),
                )
                .unwrap();
            self.motor_front_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(PULSE_NEUTRAL_US),
                )
                .unwrap();
            self.motor_back_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(PULSE_NEUTRAL_US),
                )
                .unwrap();
            self.motor_back_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(PULSE_NEUTRAL_US),
                )
                .unwrap();
        }

        if motion < 0 {
            let mut set_range = default_map(motion.clone());

            self.motor_front_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
            self.motor_front_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();

            set_range = default_map(motion.clone().abs());

            self.motor_back_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
            self.motor_back_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
        }

        if motion > 0 {
            let mut set_range = default_map(motion.clone());

            self.motor_front_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
            self.motor_front_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();

            let inverse_motion = motion - (motion * 2);

            set_range = default_map(inverse_motion);

            self.motor_back_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
            self.motor_back_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
        }
    }

    fn set_roll(&mut self, motion: i8) {
        if self.motor_state == MotorState::Disarmed {
            return;
        }

        if motion == 0 {
            self.motor_front_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(PULSE_NEUTRAL_US),
                )
                .unwrap();
            self.motor_front_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(PULSE_NEUTRAL_US),
                )
                .unwrap();
            self.motor_back_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(PULSE_NEUTRAL_US),
                )
                .unwrap();
            self.motor_back_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(PULSE_NEUTRAL_US),
                )
                .unwrap();
        }

        if motion < 0 {
            let mut set_range = default_map(motion.clone());

            self.motor_front_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
            self.motor_back_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();

            set_range = default_map(motion.clone().abs());

            self.motor_front_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
            self.motor_back_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
        }

        if motion > 0 {
            let mut set_range = default_map(motion.clone());

            self.motor_front_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
            self.motor_back_left
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();

            let inverse_motion = motion - (motion * 2);

            set_range = default_map(inverse_motion);

            self.motor_front_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
            self.motor_back_right
                .set_pwm(
                    Duration::from_millis(PERIOD_MS),
                    Duration::from_micros(set_range.clone()),
                )
                .unwrap();
        }
    }

    fn set_yaw(&mut self, motion: i8) {
        if self.motor_state == MotorState::Disarmed {
            return;
        }

        let set_range = default_map(motion.clone());

        self.motor_front_turn
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(set_range.clone()),
            )
            .unwrap();
    }

    fn set_acceleration(&mut self, motion: i8) {
        if self.motor_state == MotorState::Disarmed {
            return;
        }

        let set_range = default_map(motion.clone());

        self.motor_back_thrust
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(set_range.clone()),
            )
            .unwrap();
    }

    fn set_all(&mut self, acceleration: i8, yaw: i8, pitch: i8, roll: i8) {
        if self.motor_state == MotorState::Disarmed {
            return;
        }

        self.set_acceleration(acceleration);
        self.set_yaw(yaw);
        self.set_pitch(pitch);
        self.set_roll(roll);
    }

    fn set_all_same(&mut self, motion: i8) {
        if self.motor_state == MotorState::Disarmed {
            return;
        }

        self.set_acceleration(motion);
        self.set_yaw(motion);
        self.set_pitch(motion);
        self.set_roll(motion);
    }

    fn disarm(&mut self) {
        self.motor_front_left.clear_pwm();
        self.motor_front_right.clear_pwm();
        self.motor_back_left.clear_pwm();
        self.motor_back_right.clear_pwm();
        self.motor_front_turn.clear_pwm();
        self.motor_back_thrust.clear_pwm();
        self.motor_state = MotorState::Disarmed;
    }

    fn arm(&mut self) {
        self.motor_front_left
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();
        self.motor_front_right
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();
        self.motor_back_left
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();
        self.motor_back_right
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();
        self.motor_front_turn
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();
        self.motor_back_thrust
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();

        sleep(Duration::from_millis(1000));

        self.motor_front_left
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_MAX_US),
            )
            .unwrap();
        self.motor_front_right
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_MAX_US),
            )
            .unwrap();
        self.motor_back_left
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_MAX_US),
            )
            .unwrap();
        self.motor_back_right
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_MAX_US),
            )
            .unwrap();
        self.motor_front_turn
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_MAX_US),
            )
            .unwrap();
        self.motor_back_thrust
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_MAX_US),
            )
            .unwrap();

        sleep(Duration::from_millis(1000));

        self.motor_front_left
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();
        self.motor_front_right
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();
        self.motor_back_left
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();
        self.motor_back_right
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();
        self.motor_front_turn
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();
        self.motor_back_thrust
            .set_pwm(
                Duration::from_millis(PERIOD_MS),
                Duration::from_micros(PULSE_NEUTRAL_US),
            )
            .unwrap();
        self.motor_state = MotorState::Armed;
    }
}
