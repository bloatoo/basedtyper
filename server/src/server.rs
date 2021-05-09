use std::sync::Arc;
use serde_json::{Value, json};
use tokio::sync::Mutex;
use crate::message::Message;

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

    pub async fn process_message(&mut self, message_string: String, username: String) {
        match Message::from(message_string.clone()) {
            Message::Keypress => {
                let message: Value = json!({
                    "call": "keypress",
                    "username": username,
                });

                self.forward(message.to_string(), username).await;
            }

            Message::Finished => {
                println!("{} finished", username);
            }

            _ => println!("invalid message: {}", message_string)
        }
    }

    /*pub async fn construct_message<T: ToString>(&self, call: String, username: String, data: T) -> Value {
        json!({
            "call": call,
            "username": username,
            "data": data.to_string(),
        })
    }*/

    /*pub async fn validate_message(message: String) -> Result<Message, Box<dyn std::error::Error>> {
    }*/
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
