use crate::client::message::Message;

use crate::client::{app::{App, Player, State}, message::ServerMessage, ui::wordlist::Wordlist};

use std::sync::Arc;
use tokio::{io::AsyncWriteExt, sync::Mutex};
use super::IOEvent;

pub struct EventHandler {
    app: Arc<Mutex<App>>,
}

impl EventHandler {
    pub fn new(app: Arc<Mutex<App>>) -> Self {
        Self {
            app,
        }
    }

    pub async fn handle_event(&mut self, event: IOEvent) {
        let mut app = self.app.lock().await;
        let sock = app.connection.tcp.clone().unwrap();
        let mut sock_lock = sock.lock().await;

        match event {
            IOEvent::Keypress(wpm) => {
                sock_lock.write(Message::Keypress(wpm).to_string().as_bytes()).await.unwrap();
            }

            IOEvent::ServerMessage(msg) => {
                match ServerMessage::from(msg) {
                    ServerMessage::Start(words) => {
                        let wordlist = Wordlist::from(words);

                        app.wordlist = wordlist;

                        app.restart(State::TypingGame);
                    }

                    ServerMessage::Init(data) => {
                        let data: Vec<Player> = data.iter().map(|p| Player::new(p.username.clone(), p.color.clone())).collect();
                        app.set_players(data);
                    }

                    ServerMessage::Keypress((username, wpm)) => {
                        let player = app.connection.players.iter_mut().find(|p| p.username == username).unwrap();
                        player.wpm = wpm;
                    }

                    ServerMessage::Join(data) => {
                        app.connection.players.push(Player::new(data.username, data.color));
                    }

                    ServerMessage::Finished(username) => {
                        app.connection.players.iter_mut().find(|p| p.username == username).unwrap().finished = true;
                    }

                    _ => (),
                }
            }
        }
    }
}
