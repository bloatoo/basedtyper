use server::{server::Server, client::Client};
use std::{io::{self, Read, Write}, net::TcpListener, sync::mpsc::{Sender, Receiver}, thread}; use std::sync::{Arc, Mutex};

use std::sync::mpsc;

use serde_json::{json, Value};

fn nonblocking_stdin() -> Receiver<String> {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        sender.send(buf).unwrap();
    });
    receiver
}
fn main() { let (sender, receiver) = mpsc::channel::<String>(); let input = nonblocking_stdin();

    let port = std::env::args().nth(1).unwrap_or(String::from("1337"));
    let port = port.parse::<u32>().unwrap_or(1337);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    let clients: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));

    listener.set_nonblocking(true).unwrap();

    println!("Server started on port {}.", port);

    loop {
        if let Ok(_msg) = receiver.try_recv() {
            
        }

        if let Ok(_data) = input.try_recv() {
            
        }

        if let Ok((mut stream, _)) = listener.accept() {
            let clients = clients.clone();
            let sender = sender.clone();
            std::thread::spawn(move || {
                let mut buf = vec![0u8; 1024];

                if let Err(e) = stream.read(&mut buf) {
                    println!("Failed to read from stream: {}", e.to_string());
                }

                buf.retain(|byte| byte != &u8::MIN);

                if !buf.is_empty() {
                    let message = String::from_utf8(buf).unwrap();

                    let json: Value = serde_json::from_str(&message).unwrap();

                    let call = json["call"].as_str();

                    if let Some(call) = call {
                        if call == "init" {
                            let username = String::from(json["data"]["username"].as_str().unwrap_or("anonymous"));

                            println!("new client with username {}", username);

                            let mut clients = clients.lock().unwrap();
                            clients.push(Client::new(stream.try_clone().unwrap(), username));
                        }
                    }

                    sender.send(message).unwrap();
                }

            });
        }

    }
}
