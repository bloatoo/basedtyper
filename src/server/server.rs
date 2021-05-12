use std::sync::Arc;
use serde_json::{Value, json};
use tokio::sync::Mutex;
use super::message::{Color, Forwardable, Message, UserData};

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

    pub async fn create_init_message(&self) -> String {
        let data = self.clients.lock().await.iter().map(|client| {
            serde_json::from_str(&UserData::new(client.username.clone(), Color::from(&client.color[..])).to_string()[..]).unwrap()
        }).collect::<Vec<Value>>();

        let data = json!({
            "call": "init",
            "data": {
                "players": Value::Array(data)
            }
        }).to_string();
        data
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

    pub async fn process_message(&mut self, message_string: String, username: String) {
        match Message::from(message_string.clone().as_str()) {
            Message::Keypress => {
                let msg = Message::Keypress.forwardable(username.clone());
                self.forward(msg, username).await;
            }

            Message::Finished(wpm) => {
                let msg = Message::Finished(wpm).forwardable(username.clone());
                println!("{} finished with {} as their WPM", username.clone(), wpm);
                self.forward(msg, username).await;
            }

            _ => println!("invalid message: {}", message_string)
        }
    }

    pub async fn forward(&mut self, data: String, username: String) {
        let mut clients = self.clients.lock().await;

        for i in 0..clients.len() {
            if i >= clients.len() {
                continue;
            }

            let client = &mut clients[i];
            if client.username != username {
                if let Err(e) = client.writer.write(data.to_string().as_bytes()).await {
                    clients.remove(i);
                    println!("error: {}", e);
                }
            }
        }
    }
}
