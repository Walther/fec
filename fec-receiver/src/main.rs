use clap::{App, Arg};
use std::net::UdpSocket;
use std::str::from_utf8;

fn main() -> std::io::Result<()> {
    {
        let matches = App::new("fec-receiver")
            .version("0.1")
            .about("Forward error correction demo, receiver application")
            .arg(
                Arg::with_name("receiver")
                    .short("r")
                    .long("receiver")
                    .value_name("RECEIVER")
                    .help("Sets the receiver address to listen on")
                    .required(true)
                    .takes_value(true),
            )
            .get_matches();
        let receiver = matches
            .value_of("receiver")
            .expect("Unable to parse receiver address");
        let socket = UdpSocket::bind(receiver)?;

        loop {
            // Receives a 100 byte datagram
            let mut buf = [0; 100];
            let (amt, src) = socket.recv_from(&mut buf)?;
            dbg!(amt, src);

            let packet = Packet::new(buf[0], &buf[1..]);
            dbg!(packet.id);
            let data = from_utf8(&packet.data).expect("Unable to parse data"); // Assuming utf8 sent over wire
            dbg!(data);
        }
    }
    Ok(())
}

/// A UDP data packet with a sequential id and data portion.
/// Should be 100 bytes each.
struct Packet {
    id: u8, // Sequential packet number
    data: [u8; 99],
}

impl Packet {
    fn new(id: u8, data: &[u8]) -> Packet {
        let mut p = Packet { id, data: [0; 99] };
        for (index, byte) in data.iter().enumerate() {
            p.data[index] = *byte;
        }
        p
    }

    fn to_bytes(&self) -> [u8; 100] {
        let mut raw_data = [0; 100];
        raw_data[0] = self.id;
        for (index, byte) in self.data.iter().enumerate() {
            raw_data[index + 1] = *byte;
        }
        raw_data
    }

    fn get_id(&self) -> u8 {
        self.id
    }

    fn get_data(&self) -> [u8; 99] {
        self.data
    }
}
