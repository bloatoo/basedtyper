use super::super::{app::{App, Player, State}, message::ServerMessage, ui::wordlist::Wordlist};
use std::sync::Arc;
use tokio::sync::Mutex;
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

        match event {
            IOEvent::ServerMessage(msg) => {
                match ServerMessage::from(msg) {
                    ServerMessage::Start(words) => {
                        let wordlist = Wordlist::from(words.to_string());

                        app.wordlist = wordlist;

                        app.restart(State::TypingGame);
                    }

                    ServerMessage::Init(data) => {
                        let mut data: Vec<Player> = data.iter().map(|p| Player::new(p.username.clone(), p.color.clone())).collect();
                        app.set_players(&mut data);
                    }

                    ServerMessage::Keypress(username) => {
                        let player = app.connection.players.iter_mut().find(|p| p.username == username).unwrap();
                        player.pos += 1;
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
