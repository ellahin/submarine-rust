use crate::data::threads::movechannel::{MovementChannelData, MovementChannelDataType};
use common_data::commands::movement::Movement;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc::Sender;

pub struct StreamHandler;

impl StreamHandler {
    pub fn new() -> Self {
        return StreamHandler;
    }

    pub fn handle(&self, stream: TcpStream, movement_channel: Sender<MovementChannelData>) {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = BufWriter::new(stream.try_clone().unwrap());

        let mut read_buff: Vec<u8> = Vec::new();

        let stream_end = 255;

        let _ = reader
            .read_until(stream_end, &mut read_buff)
            .expect("Could not read stream");

        // closing stream if there is less then two bytes in the buff
        if read_buff.len() < 2 {
            stream.shutdown(Shutdown::Both).unwrap();
            return;
        }
        match read_buff[0] {
            0 => StreamHandler::ping(&mut writer, stream),
            1 => StreamHandler::movement(read_buff, &mut writer, movement_channel, stream),
            _ => stream.shutdown(Shutdown::Both).unwrap(),
        }
    }

    fn ping(writer: &mut BufWriter<TcpStream>, stream: TcpStream) {
        let ping_return: [u8; 2] = [0, 255];
        println!("ping");
        writer.write_all(&ping_return).unwrap();
        writer.flush().unwrap();
        stream.shutdown(Shutdown::Both).unwrap();
    }

    fn movement(
        buff: Vec<u8>,
        writer: &mut BufWriter<TcpStream>,
        movement_channel: Sender<MovementChannelData>,
        stream: TcpStream,
    ) {
        let mut working_vec = buff;

        working_vec.pop();
        if working_vec.len() != 8 {
            stream.shutdown(Shutdown::Both).unwrap();
            return;
        }

        println!("movement buff = {:#?}", working_vec);

        let working_buff: [u8; 8] = working_vec.try_into().unwrap();

        let movement_command = Movement::decode_packet(working_buff.to_owned());

        if movement_command.is_err() {
            stream.shutdown(Shutdown::Both).unwrap();
            return;
        }

        let channel_msg = MovementChannelData {
            data_type: MovementChannelDataType::Move,
            data: Some(movement_command.unwrap()),
        };

        let chanel_status = movement_channel.send(channel_msg);

        let mut write_buff = [1, 255];

        if chanel_status.is_err() {
            write_buff = [0, 255];
        }

        writer.write_all(&write_buff).unwrap();
        writer.flush().unwrap();
        stream.shutdown(Shutdown::Both).unwrap();
        return;
    }
}
