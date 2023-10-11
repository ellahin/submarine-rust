pub struct Movment {
    // Movement order: accel, pitch, yaw, roll
    movement_command: [i8; 4],
    checksum: i16,
    packet: Option<[u8; 7]>,
}

const COMMAND_NUMBER: u8 = 1;

pub enum MovmentSetError {
    IncorrectNumber,
}
pub enum MovmentPacketDecodeError {
    ChecksumNotValid,
    NotMovementPacket,
}

impl Movment {
    pub fn new() -> Self {
        return Movment {
            movement_command: [0, 0, 0, 0],
            checksum: 0,
            packet: None,
        };
    }

    pub fn set_acceleration(&mut self, percent: i8) -> Result<(), MovmentSetError> {
        if percent < -100 || percent > 100 {
            return Err(MovmentSetError::IncorrectNumber);
        }

        self.movement_command[0] = percent;

        let _ = self.set_checksum();

        return Ok(());
    }

    pub fn set_pitch(&mut self, percent: i8) -> Result<(), MovmentSetError> {
        if percent < -100 || percent > 100 {
            return Err(MovmentSetError::IncorrectNumber);
        }

        self.movement_command[1] = percent;

        let _ = self.set_checksum();

        return Ok(());
    }

    pub fn set_yaw(&mut self, percent: i8) -> Result<(), MovmentSetError> {
        if percent < -100 || percent > 100 {
            return Err(MovmentSetError::IncorrectNumber);
        }

        self.movement_command[2] = percent;

        let _ = self.set_checksum();

        return Ok(());
    }

    pub fn set_roll(&mut self, percent: i8) -> Result<(), MovmentSetError> {
        if percent < -100 || percent > 100 {
            return Err(MovmentSetError::IncorrectNumber);
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

    pub fn decode_packet(packet: [u8; 7]) -> Result<Self, MovmentPacketDecodeError> {
        if packet[0] != COMMAND_NUMBER {
            return Err(MovmentPacketDecodeError::NotMovementPacket);
        }

        let checksum = ((packet[5] as i16) << 8) | packet[6] as i16;

        let movement = [
            packet[1] as i8,
            packet[2] as i8,
            packet[3] as i8,
            packet[4] as i8,
        ];

        let mut working_movement = Movment {
            movement_command: movement,
            checksum: checksum.clone(),
            packet: Some(packet),
        };

        working_movement.set_checksum();

        if working_movement.checksum != checksum {
            return Err(MovmentPacketDecodeError::ChecksumNotValid);
        }

        return Ok(working_movement);
    }
}
