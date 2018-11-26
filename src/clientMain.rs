use std::net::UdpSocket;
use std::{str,io};

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8000").expect("Err: Could not bind socekt");
    
    socket.connect("127.0.0.1:8888")
          .expect("Could not connect to server");

    loop {
        let mut input = String::new();
        let mut buffer = [0u8; 1500];

        io::stdin().read_line(&mut input)
                   .expect("Failed to read line");

        socket.send(input.as_bytes())
              .expect("Failed to send to server");
        
        socket.recv_from(&mut buffer)
              .expect("Could not read to buffer");

        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer"));
    }
}