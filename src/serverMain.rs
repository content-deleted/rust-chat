use std::thread;
use std::net::UdpSocket;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Err: Could not bind socekt");
    let users = load_accounts().expect("Could not load users");

    for user in users {
        println!("Username: {}",user.username);
        println!("Password: {}",user.password);
    }

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

fn load_accounts() -> std::io::Result<Vec<User>> {
    let file = File::open("accounts.txt")?;
    
    let reader = BufReader::new(file);

    let mut users = Vec::new();

    for lines in reader.lines()
    {
        let mut line = lines.unwrap_or(String::from("ERR"));
        let num = line.find(' ').unwrap_or(line.len());
        let mut pass = line.split_off(num);
        pass.remove(0);
        users.push(User { username: line, password: pass });
    }
    return std::result::Result::Ok(users);
}

struct User {
    username: String,
    password: String,
}