use std::sync::{Arc, Mutex};
use super::client::Client;
use super::word::Word;
use std::io::Write;

pub struct Server {
    pub clients: Arc<Mutex<Vec<Client>>>,
    pub words: Vec<Word>,
}

impl Server {
    pub fn default() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
            words: Vec::new(),
        }
    }

    pub fn broadcast(&mut self, data: String) {
        let mut clients = self.clients.lock().unwrap();

        for i in 0..clients.len() {
            println!("in broadcast loop, data: {}", data);
            let client = &mut clients[i];

            if let Err(e) = client.tcp.write(data.as_bytes()) {
                clients.remove(i);
                println!("error: {}", e);
            }
        }
    }
}
