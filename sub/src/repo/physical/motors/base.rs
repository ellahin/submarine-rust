#[derive(PartialEq, Eq)]
pub enum MotorState {
    Armed,
    Disarmed,
}

pub trait Motor {
    fn new() -> Self;
    fn set_acceleration(&mut self, motion: i8);
    fn set_yaw(&mut self, motion: i8);
    fn set_pitch(&mut self, motion: i8);
    fn set_roll(&mut self, motion: i8);
    fn set_all_same(&mut self, motion: i8);
    fn set_all(&mut self, acceleration: i8, yaw: i8, pitch: i8, roll: i8);
    fn disarm(&mut self);
    fn arm(&mut self);
}
