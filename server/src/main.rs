use server::client::Client;
use server::app::Server;

use std::{
    net::TcpListener,
    io::{Read, Write}
};

use std::sync::mpsc;

use serde_json::{json, Value};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let server = Server::default();
    let clients_clone = server.clients.clone();

    let (sender, receiver) = mpsc::channel::<String>();
    
    let port = if args.len() > 1 {
        args[1].parse::<u32>().expect("failed to parse port from argument")
    } else {
        1337
    };

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("failed to bind");

    listener.set_nonblocking(true).expect("failed");

    println!("server started on port {}", port);

    loop {
        if let Ok(data) = receiver.try_recv() {
            let mut clients = clients_clone.lock().unwrap();

            for idx in 0..clients.len() {
                let client = &mut clients[idx];

                if client.tcp.write(data.as_bytes()).is_err() {
                    clients.remove(idx);
                }
            }
        }

        if let Ok((mut stream, _)) = listener.accept() {
            println!("new connection: {}", stream.peer_addr().unwrap());
  
            let sender = sender.clone();

            let mut clients = server.clients.lock().unwrap();

            let client = Client::new(stream.try_clone().unwrap(), "".to_string());

            clients.push(client);

            std::thread::spawn(move || loop {
                let mut buf = vec![0 as u8; 1024];
 
                if let Err(err) = stream.read(&mut buf) {
                    println!("error: {}", err.to_string());
                }
                
                buf.retain(|byte| byte != &u8::MIN);

                let data = String::from_utf8(buf).unwrap();

                if data.len() > 0 {
                    let json: Value = serde_json::from_str(&data).unwrap();

                    let username = &json["data"]["username"].as_str().unwrap();

                    let json = json!({
                        "call": "words",
                        "data": {
                            "words": "these are some random words",
                        }
                    });

                    let data = serde_json::to_string(&json).unwrap();

                    sender.send(data).unwrap();

                    println!("new client with username {}", username);
                }

            });
        }
    }
}
