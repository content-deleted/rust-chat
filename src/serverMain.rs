use std::thread;
use std::net::UdpSocket;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::{str,io};

mod sharedStructs;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8888").expect("Err: Could not bind socekt");
    let users = load_accounts().expect("Could not load users");
    
    for user in users {
        println!("Username: {}",user);
        //println!("Username: {}",user.username);
        //println!("Password: {}",user.password);
    }



    loop {
        let mut buf = [0u8; 1500];
        let sock = socket.try_clone().expect("Failed to clone socekt");
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                thread::spawn(move || {
                    //println!("Handling connection from {}", src );
                    
                    let temp = str::from_utf8(&buf).unwrap().to_string();
                    println!("Received {}", temp);
                    let (messageType, payload) = temp.split_at(temp.find(" ").unwrap_or(0));
                    // match on message type so we know what to do
                    match messageType {
                        "clientLogin" => {
                            // this is where validation happens
                            //let (username, password) = payload.split_at(payload.find(" ").unwrap_or(0));
                            let mut correctPwd = false;
                            let users = load_accounts().expect("Could not load users");
                            for user in users {
                                if ( user == payload) {
                                    correctPwd = true;
                                }
                            }

                            sock.send_to( (if(correctPwd){"LoginSuccess"} else {"LoginSuccess"}).as_bytes(), &src).expect("Failed to send response"); 
                        }
                        _ => {
                            sock.send_to(String::from("FAILURE").as_bytes(), &src).expect("Failed to send response"); 
                        }
                    }
                });
            },
            Err(e) => {
                eprintln!("Could not recieve a datagram: {}", e);
            }
        }
    }
}

fn load_accounts() -> std::io::Result<Vec<String>> { //std::io::Result<Vec<sharedStructs::User>> {
    let file = File::open("accounts.txt")?;
    
    let reader = BufReader::new(file);

    let mut users = Vec::new();

    for lines in reader.lines()
    {
        let mut line = lines.unwrap_or(String::from("ERR"));
        //let num = line.find(' ').unwrap_or(line.len());
        //let mut pass = line.split_off(num);
        //pass.remove(0);
        users.push(line);//sharedStructs::User { username: line, password: pass });
    }
    return std::result::Result::Ok(users);
}