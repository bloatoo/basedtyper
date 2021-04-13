use server::{server::Server, client::Client};

use std::{io::{self, Read, Write}, net::TcpListener, sync::mpsc::{Sender, Receiver}, thread};

use std::sync::{Arc, Mutex};

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

fn start_server(port: u32, main: Sender<String>) {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("failed to bind");

    let clients: Arc<Mutex<Vec<Client>>> = Arc::new(Mutex::new(Vec::new()));

    let (sender, receiver) = mpsc::channel();

    listener.set_nonblocking(true).expect("failed");
    
    println!("server started");

    loop {
        if let Ok(_data) = receiver.try_recv() {
            
        }
        if let Ok((mut stream, _)) = listener.accept() {
            println!("new connection: {}", stream.peer_addr().unwrap());
    
            let sender = sender.clone();
    
            let clients = clients.clone();

            std::thread::spawn(move || loop {
                let mut buf = vec![0 as u8; 1024];
    
                if let Err(err) = stream.read(&mut buf) {
                    println!("error: {}", err.to_string());
                }
                    
                buf.retain(|byte| byte != &u8::MIN);
    
                let data = String::from_utf8(buf).unwrap();
    
    
                if data.len() > 0 {
                    let json: Value = serde_json::from_str(&data).unwrap();
    
                    match json["call"].as_str().unwrap() {
                        "init" => {
                            let mut clients = clients.lock().unwrap();
                            let username = &json["data"]["username"].as_str().unwrap();
    
                            let client = Client::new(stream.try_clone().unwrap(), username.to_string());
                            clients.push(client);
                            println!("new client with username {}", username);
    
                            let json = json!({
                                "call": "words",
                                "data": {
                                    "words": "these are some random words",
                                }
                            });
    
                            let data = serde_json::to_string(&json).unwrap();
        
                            sender.send(data).unwrap();
                        }
                        
                        _ => ()
                    }   
        
                }
    
            });
        }   
    }
}   

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (sender, receiver) = mpsc::channel::<String>();
    
    let port = if args.len() > 1 {
        args[1].parse::<u32>().expect("failed to parse port from argument")
    } else {
        1337
    };

    let input = nonblocking_stdin();

    loop {
        if let Ok(data) = receiver.try_recv() {
        }

        if let Ok(data) = input.try_recv() {
            println!("{}", data);
            let args: Vec<&str> = data.split(" ").collect();
            //let port = args[1].parse::<u32>().unwrap();
            match args[0] {
                "start" => {
                    let sender = sender.clone();
                    thread::spawn(move || start_server(1337, sender));
                }
                _ => ()
            }
        }

    }
}
