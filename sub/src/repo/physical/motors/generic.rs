use crate::repo::physical::motors::base::Motor;

pub struct MotorGeneric {
    acceleration: i8,
    yaw: i8,
    pitch: i8,
    roll: i8,
}

impl Motor for MotorGeneric {
    fn new() -> Self {
        return MotorGeneric {
            acceleration: 0,
            yaw: 0,
            pitch: 0,
            roll: 0,
        };
    }

    fn set_acceleration(&mut self, motion: i8) {
        println!("{}", &motion);
        self.acceleration = motion;
    }

    fn set_yaw(&mut self, motion: i8) {
        println!("{}", &motion);
        self.yaw = motion;
    }

    fn set_pitch(&mut self, motion: i8) {
        println!("{}", &motion);
        self.pitch = motion;
    }

    fn set_roll(&mut self, motion: i8) {
        println!("{}", &motion);
        self.roll = motion;
    }

    fn set_all_same(&mut self, motion: i8) {
        self.acceleration = motion.clone();
        self.yaw = motion.clone();
        self.pitch = motion.clone();
        self.roll = motion.clone();
        println!("All motions set to {}", motion);
    }

    fn set_all(&mut self, acceleration: i8, yaw: i8, pitch: i8, roll: i8) {
        self.acceleration = acceleration;
        self.yaw = yaw;
        self.pitch = pitch;
        self.roll = roll;
        println!(
            "Motors set to accel: {}, yaw: {}, pitch: {}, roll: {}",
            self.acceleration, self.yaw, self.pitch, self.roll
        );
    }

    fn disengage(&mut self) {
        println!("Motors disengaged");
    }
}
