use crate::{app::{App, Player, State}, ui::wordlist::Wordlist};
use super::ServerMessage::{self, *};

pub async fn message_handler(msg_string: String, app: &mut App) {
    let msg = ServerMessage::from(msg_string.clone());

    match msg {
        Start(words) => {
            let wordlist = Wordlist::from(words.to_string());

            app.wordlist = wordlist;

            app.restart(State::TypingGame);
        }

        Init(data) => {
            let mut data: Vec<Player> = data.iter().map(|p| Player::new(p.username.clone(), p.color.clone())).collect();
            app.set_players(&mut data);
        }

        Keypress(username) => {
            let player = app.connection.players.iter_mut().find(|p| p.username == username).unwrap();
            player.pos += 1;
        }

        Join(data) => {
            app.connection.players.push(Player::new(data.username, data.color));
        }

        Finished(username) => {
            app.connection.players.iter_mut().find(|p| p.username == username).unwrap().finished = true;
        }

        _ => panic!("unknown message, {:#?}", msg_string)
    }
}
