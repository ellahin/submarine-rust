use common_data::commands::movement::Movement;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MovementChannelDataType {
    Move,
    CheckTimeout,
}

#[derive(Debug, Clone)]
pub struct MovementChannelData {
    pub data_type: MovementChannelDataType,
    pub data: Option<Movement>,
}
