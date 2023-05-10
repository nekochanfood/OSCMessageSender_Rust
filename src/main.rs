extern crate rosc;

use std::net::{UdpSocket, SocketAddr};
use rosc::{OscPacket, OscMessage, OscType};
use std::io::{self, Write};

static IP: &str = "127.0.0.1";
static PORT: u16 = 9000;

fn main() {
    loop {
        let str: String = input("> ".to_string());

        let message = OscMessage {
            addr: "/chatbox/input".to_string(),
            args: vec![
                OscType::String(str.to_string()),
                OscType::Bool(true),
            ]
        };
        
        send(message);
    }
}

fn input(comment: String) -> String{
    print!("{}",comment);
    io::stdout().flush().unwrap();

    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    return str;
}

fn send(message: OscMessage){
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let addr = SocketAddr::new(IP.parse().unwrap(), PORT);
        
    let packet = OscPacket::Message(message);
    let encoded_packet = rosc::encoder::encode(&packet).unwrap();

    socket.send_to(&encoded_packet, addr).unwrap();
}