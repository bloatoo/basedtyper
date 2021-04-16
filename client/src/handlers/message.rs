use crate::{app::{App, State}, parser::Word};
use serde_json::Value;

pub fn message_handler(msg: String, app: &mut App) {
    let json: Value = serde_json::from_str(&msg).unwrap();
    let call = json["call"].as_str().unwrap();

    match call {
        "start" => {
            let words = json["data"]["words"].as_str().unwrap();
            let words1: Vec<&str> = words
                .split(' ')
                .collect();

            let words = words1.clone().iter()
                .map(|word| Word::new(word, &""))
                .collect();
            app.words = words;

            app.word_string = words1.join(" ");

            app.restart(State::TypingGame);
        }

        _ => ()
    }
}
