use common_data::commands::movement::Movement;
use std::io::{BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
fn main() {
    let mut stream_raw = TcpStream::connect("127.0.0.1:3000").unwrap();
    let mut stream = BufWriter::new(stream_raw.try_clone().unwrap());

    let mut command = Movement::new();

    command.set_yaw(-100).expect("invalid yaw input");

    command.set_acceleration(100).expect("invalid accel input");

    let movement_buff = command.generate_packet();

    let mut buff_end: Vec<u8> = [255].to_vec();

    let mut buff: Vec<u8> = movement_buff.to_vec();

    buff.append(&mut buff_end);

    stream.write_all(&buff).expect("Failed to write to server");

    stream.flush().unwrap();

    let mut reader = BufReader::new(stream_raw);

    let mut buff: Vec<u8> = Vec::new();

    reader.read_to_end(&mut buff).unwrap();

    println!("buff: {:#?}", buff);
}
