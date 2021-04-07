use std::sync::{Arc, Mutex};
use super::client::Client;
use super::word::Word;

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
}
