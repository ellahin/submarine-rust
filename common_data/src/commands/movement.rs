#[derive(Debug, Clone)]
pub struct Movement {
    // Movement order: accel, pitch, yaw, roll
    movement_command: [i8; 4],
    checksum: i16,
    packet: Option<[u8; 7]>,
}

const COMMAND_NUMBER: u8 = 1;

#[derive(Debug, Clone)]
pub enum MovementSetError {
    IncorrectNumber,
}
#[derive(Debug, Clone)]
pub enum MovementPacketDecodeError {
    ChecksumNotValid,
    NotMovementPacket,
}

impl Movement {
    pub fn new() -> Self {
        return Movement {
            movement_command: [0, 0, 0, 0],
            checksum: 0,
            packet: None,
        };
    }

    pub fn set_acceleration(&mut self, percent: i8) -> Result<(), MovementSetError> {
        if percent < -100 || percent > 100 {
            return Err(MovementSetError::IncorrectNumber);
        }

        self.movement_command[0] = percent;

        let _ = self.set_checksum();

        return Ok(());
    }

    pub fn set_pitch(&mut self, percent: i8) -> Result<(), MovementSetError> {
        if percent < -100 || percent > 100 {
            return Err(MovementSetError::IncorrectNumber);
        }

        self.movement_command[1] = percent;

        let _ = self.set_checksum();

        return Ok(());
    }

    pub fn set_yaw(&mut self, percent: i8) -> Result<(), MovementSetError> {
        if percent < -100 || percent > 100 {
            return Err(MovementSetError::IncorrectNumber);
        }

        self.movement_command[2] = percent;

        let _ = self.set_checksum();

        return Ok(());
    }

    pub fn set_roll(&mut self, percent: i8) -> Result<(), MovementSetError> {
        if percent < -100 || percent > 100 {
            return Err(MovementSetError::IncorrectNumber);
        }

        self.movement_command[3] = percent;

        let _ = self.set_checksum();

        return Ok(());
    }

    pub fn set_checksum(&mut self) {
        let mut checksum: i16 = 0;

        for number in self.movement_command.iter() {
            checksum = checksum + i16::from(number.to_owned());
        }

        // adding command number

        checksum = checksum + i16::from(COMMAND_NUMBER);

        self.checksum = checksum;
    }

    pub fn generate_packet(&mut self) -> [u8; 7] {
        let checksum_bytes: [u8; 2] = self.checksum.to_be_bytes();

        let created_packet: [u8; 7] = [
            COMMAND_NUMBER,
            self.movement_command[0].to_be_bytes()[0],
            self.movement_command[1].to_be_bytes()[0],
            self.movement_command[2].to_be_bytes()[0],
            self.movement_command[3].to_be_bytes()[0],
            checksum_bytes[0],
            checksum_bytes[1],
        ];

        self.packet = Some(created_packet.clone());

        return created_packet;
    }

    pub fn decode_packet(packet: [u8; 7]) -> Result<Self, MovementPacketDecodeError> {
        if packet[0] != COMMAND_NUMBER {
            return Err(MovementPacketDecodeError::NotMovementPacket);
        }

        let checksum = ((packet[5] as i16) << 8) | packet[6] as i16;

        let movement = [
            packet[1] as i8,
            packet[2] as i8,
            packet[3] as i8,
            packet[4] as i8,
        ];

        let mut working_movement = Movement {
            movement_command: movement,
            checksum: checksum.clone(),
            packet: Some(packet),
        };

        working_movement.set_checksum();

        if working_movement.checksum != checksum {
            return Err(MovementPacketDecodeError::ChecksumNotValid);
        }

        return Ok(working_movement);
    }
}
