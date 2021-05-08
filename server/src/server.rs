use std::sync::Arc;
use tokio::sync::Mutex;
use super::client::Client;
use super::word::Word;
use tokio::io::AsyncWriteExt;

#[derive(Clone)]
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

    pub async fn broadcast(&mut self, data: String) {
        let mut clients = self.clients.lock().await;

        for i in 0..clients.len() {
            let client = &mut clients[i];
            if let Err(e) = client.writer.write(data.as_bytes()).await {
                clients.remove(i);
                println!("error: {}", e);
            }

        }
    }
    pub async fn forward(&mut self, data: String, username: String) {
        let mut clients = self.clients.lock().await;

        for i in 0..clients.len() {
            let client = &mut clients[i];
            if client.username != username {
                if let Err(e) = client.writer.write(data.as_bytes()).await {
                    clients.remove(i);
                println!("error: {}", e);
                }
            }
        }
    }
}
