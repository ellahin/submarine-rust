use crate::repo::physical::motors::base::Motor;
use common_data::libs::math::maprang::map_range;

use wiringpi::pin::{Gpio, SoftPwmPin};

pub struct ServoSixFront {
    motor_front_left: SoftPwmPin<Gpio>,
    motor_front_right: SoftPwmPin<Gpio>,
    motor_back_left: SoftPwmPin<Gpio>,
    motor_back_right: SoftPwmPin<Gpio>,
    motor_front_turn: SoftPwmPin<Gpio>,
    motor_back_thrust: SoftPwmPin<Gpio>,
    map_range_in_min: i8,
    map_range_in_max: i8,
    map_range_out_min: i32,
    map_range_out_max: i32,
}

impl Motor for ServoSixFront {
    fn new() -> Self {
        let pi = wiringpi::setup_gpio();

        return ServoSixFront {
            motor_front_left: pi.soft_pwm_pin(7),
            motor_front_right: pi.soft_pwm_pin(0),
            motor_back_left: pi.soft_pwm_pin(2),
            motor_back_right: pi.soft_pwm_pin(3),
            motor_front_turn: pi.soft_pwm_pin(21),
            motor_back_thrust: pi.soft_pwm_pin(22),
            map_range_out_min: 0,
            map_range_out_max: 100,
            map_range_in_min: -100,
            map_range_in_max: 100,
        };
    }

    fn set_pitch(&mut self, motion: i8) {
        if motion == 0 {
            self.motor_front_left.pwm_write(0);
            self.motor_front_right.pwm_write(0);
            self.motor_back_left.pwm_write(0);
            self.motor_back_right.pwm_write(0);
        }

        if motion < 0 {
            let mut set_range = map_range(
                motion.clone(),
                self.map_range_in_min.clone(),
                self.map_range_in_max.clone(),
                self.map_range_out_min.clone(),
                self.map_range_out_max.clone(),
            );

            self.motor_front_left.pwm_write(set_range.clone());
            self.motor_front_right.pwm_write(set_range.clone());

            set_range = map_range(
                motion.abs(),
                self.map_range_in_min.clone(),
                self.map_range_in_max.clone(),
                self.map_range_out_min.clone(),
                self.map_range_out_max.clone(),
            );

            self.motor_back_left.pwm_write(set_range);
            self.motor_back_right.pwm_write(set_range);
        }

        if motion > 0 {
            let mut set_range = map_range(
                motion.clone(),
                self.map_range_in_min.clone(),
                self.map_range_in_max.clone(),
                self.map_range_out_min.clone(),
                self.map_range_out_max.clone(),
            );

            self.motor_front_left.pwm_write(set_range.clone());
            self.motor_front_right.pwm_write(set_range.clone());

            let inverse_motion = motion - (motion * 2);

            set_range = map_range(
                inverse_motion,
                self.map_range_in_min.clone(),
                self.map_range_in_max.clone(),
                self.map_range_out_min.clone(),
                self.map_range_out_max.clone(),
            );

            self.motor_back_left.pwm_write(set_range);
            self.motor_back_right.pwm_write(set_range);
        }
    }

    fn set_roll(&mut self, motion: i8) {
        if motion == 0 {
            self.motor_front_left.pwm_write(0);
            self.motor_front_right.pwm_write(0);
            self.motor_back_left.pwm_write(0);
            self.motor_back_right.pwm_write(0);
        }

        if motion < 0 {
            let mut set_range = map_range(
                motion.clone(),
                self.map_range_in_min.clone(),
                self.map_range_in_max.clone(),
                self.map_range_out_min.clone(),
                self.map_range_out_max.clone(),
            );

            self.motor_front_left.pwm_write(set_range.clone());
            self.motor_back_left.pwm_write(set_range.clone());

            set_range = map_range(
                motion.abs(),
                self.map_range_in_min.clone(),
                self.map_range_in_max.clone(),
                self.map_range_out_min.clone(),
                self.map_range_out_max.clone(),
            );

            self.motor_front_right.pwm_write(set_range);
            self.motor_back_right.pwm_write(set_range);
        }

        if motion > 0 {
            let mut set_range = map_range(
                motion.clone(),
                self.map_range_in_min.clone(),
                self.map_range_in_max.clone(),
                self.map_range_out_min.clone(),
                self.map_range_out_max.clone(),
            );

            self.motor_front_left.pwm_write(set_range.clone());
            self.motor_back_left.pwm_write(set_range.clone());

            let inverse_motion = motion - (motion * 2);

            set_range = map_range(
                inverse_motion,
                self.map_range_in_min.clone(),
                self.map_range_in_max.clone(),
                self.map_range_out_min.clone(),
                self.map_range_out_max.clone(),
            );

            self.motor_front_right.pwm_write(set_range);
            self.motor_back_right.pwm_write(set_range);
        }
    }

    fn set_yaw(&mut self, motion: i8) {
        let mut set_range = map_range(
            motion,
            self.map_range_in_min.clone(),
            self.map_range_in_max.clone(),
            self.map_range_out_min.clone(),
            self.map_range_out_max.clone(),
        );

        self.motor_front_turn.pwm_write(set_range);
    }

    fn set_acceleration(&mut self, motion: i8) {
        let mut set_range = map_range(
            motion,
            self.map_range_in_min.clone(),
            self.map_range_in_max.clone(),
            self.map_range_out_min.clone(),
            self.map_range_out_max.clone(),
        );

        self.motor_back_thrust.pwm_write(set_range);
    }

    fn set_all(&mut self, acceleration: i8, yaw: i8, pitch: i8, roll: i8) {
        self.set_acceleration(acceleration);
        self.set_yaw(yaw);
        self.set_pitch(pitch);
        self.set_roll(roll);
    }

    fn set_all_same(&mut self, motion: i8) {
        self.set_acceleration(motion);
        self.set_yaw(motion);
        self.set_pitch(motion);
        self.set_roll(motion);
    }

    fn disengage(&mut self) {
        self.set_all(0, 0, 0, 0);
    }
}
