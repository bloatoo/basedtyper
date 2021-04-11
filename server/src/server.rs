use std::sync::{Arc, Mutex};
use super::client::Client;
use super::word::Word;
use std::io::Write;

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

    pub fn call(&mut self, data: String) {
        let mut clients = self.clients.lock().unwrap();

        for i in 0..clients.len() {
            let client = &mut clients[i];

            if client.tcp.write(data.as_bytes()).is_err() {
                clients.remove(i);
            }
        }
    }
}
