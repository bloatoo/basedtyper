use crate::app::App;
use serde_json::Value;

pub fn message_handler(msg: String, app: &mut App) {
    let json: Value = serde_json::from_str(&msg).unwrap();
    match json["call"].as_str().unwrap() {
        _ => ()
    }
}
