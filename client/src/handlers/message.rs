use crate::{app::{App, State}, ui::wordlist::Wordlist};
use serde_json::Value;

pub fn message_handler(msg: String, app: &mut App) {
    let json: Value = serde_json::from_str(&msg).unwrap();
    let call = json["call"].as_str().unwrap();

    match call {
        "start" => {
            let words = json["data"]["words"].as_str().unwrap();

            let wordlist = Wordlist::from(words.to_string());

            app.wordlist = wordlist;

            app.restart(State::TypingGame);
        }

        _ => ()
    }
}
