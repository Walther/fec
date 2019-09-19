use clap::{App, Arg};
use rand::prelude::*;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let matches = App::new("fec-sender")
            .version("0.1")
            .about("Forward error correction demo, sender application")
            .arg(
                Arg::with_name("sender")
                    .short("s")
                    .long("sender")
                    .value_name("SENDER")
                    .help("Sets the sender address to bind")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("receiver")
                    .short("r")
                    .long("receiver")
                    .value_name("RECEIVER")
                    .help("Sets the receiver address to send to")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("target_loss")
                    .short("l")
                    .long("target_loss")
                    .value_name("TARGET_LOSS")
                    .help("Sets the target packet loss. Implemented with pseudorandom weighted coinflip for each datagram")
                    .takes_value(true),
            )
            .get_matches();

        let receiver = matches
            .value_of("receiver")
            .expect("Unable to parse receiver address");

        let sender = matches
            .value_of("sender")
            .expect("Unable to parse sender address");

        let target_loss: f32 = matches
            .value_of("target_loss")
            .unwrap_or("0.0") // default value
            .parse::<f32>()
            .unwrap();

        let mut rng = rand::thread_rng();

        let lipsum: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut nec iaculis turpis. Nullam eget commodo nunc. Praesent auctor eros in risus luctus ullamcorper. Morbi aliquam leo ac fringilla sagittis. Phasellus vel diam sed odio aliquet aliquam. Suspendisse potenti. Nunc vel euismod mi, a dapibus dolor."; // 300 characters

        let data = &lipsum.as_bytes()[..99];

        let packet = Packet::new(1, &data);

        lossy_send_data(sender, receiver, packet, target_loss, rng);
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

fn lossy_send_data(
    sender: &str,
    receiver: &str,
    packet: Packet,
    target_loss: f32,
    mut rand: rand::rngs::ThreadRng,
) -> Result<(), String> {
    let socket = UdpSocket::bind(sender).expect("Unable to bind sending address");
    let packetloss: f32 = rand.gen();
    let data = packet.to_bytes();
    if packetloss > target_loss {
        socket
            .send_to(&data, &receiver)
            .expect("Error while sending data");
    }

    Ok(())
}
