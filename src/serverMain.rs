use std::thread;
use std::net::UdpSocket;

fn main() {
    
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Err: Could not bind socekt");
    
    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socekt");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    println!("Handling connection from {}", src );
                    sock.send_to(&buf, &src).expect("Failed to send response");
                });
            },
            Err(e) => {
                eprintln!("Could not recieve a datagram: {}", e);
            }
        }
    }
}

/*
handle_client().unwrap_or_else(|error| eprintln!("{:?}", error));

fn handle_client () -> Result<(), Error> {
    println!("Handling connection from {}", src );
    sock.send_to(&buf, &src).expect("Failed to send response");
}
*/