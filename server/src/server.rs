use crate::{word::Word, input::{nonblocking_stdin, input_handler}, client::Client};

use std::{io::{self, Read, Write}, net::TcpListener, sync::{Arc, Mutex, mpsc::{channel, Receiver}}, thread};

use serde_json::{json, Value};

pub struct Server {
    pub clients: Arc<Mutex<Vec<Client>>>,
    pub game_started: bool,
    pub words: Vec<Word>,
}

impl Server {
    pub fn default() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
            game_started: false,
            words: Vec::new(),
        }
    }

    pub fn broadcast(&self, data: String) {
        let mut clients = self.clients.lock().unwrap();

        for i in 0..clients.len() {
            let client = &mut clients[i];

            if client.tcp.write(data.as_bytes()).is_err() {
                clients.remove(i);
            }
        }
    }

    pub fn start(sender: std::sync::mpsc::Sender<String>, port: u32) -> ! {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("failed to bind");

        listener.set_nonblocking(true).expect("failed");

        let stdin: Receiver<String> = nonblocking_stdin();

        println!("server started on port {}", port);

        let (sender, receiver) = channel::<String>();

        loop {
            if let Ok(message) = stdin.try_recv() {
                //input_handler(message, self);
            }
            
            if let Ok(data) = receiver.try_recv() {
            }

            if let Ok((mut stream, _)) = listener.accept() {
                println!("new connection: {}", stream.peer_addr().unwrap());
    
                let sender = sender.clone();
    
                //let clients = self.clients.clone();
    
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
                                //let mut clients = clients.lock().unwrap();
                                let username = &json["data"]["username"].as_str().unwrap();
    
                                let client = Client::new(stream.try_clone().unwrap(), username.to_string());
                                //clients.push(client);
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
}
