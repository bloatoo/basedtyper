use std::sync::{Arc, Mutex};
use super::client::Client;

pub struct Server {
    pub clients: Arc<Mutex<Vec<Client>>>,
}

impl Server {
    pub fn default() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
