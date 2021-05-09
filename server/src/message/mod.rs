pub mod user_data;
use serde_json::Value;
pub use user_data::{UserData, Color};

pub enum Message {
    Join(UserData),
    Keypress/*(String)*/,
    Finished,
    Unknown,
}

impl<S: ToString> From<S> for Message {
    fn from(data: S) -> Self {
        let value = serde_json::from_str(&data.to_string());

        if let Err(_) = value {
            return Message::Unknown;
        }

        let value: Value = value.unwrap();

        let data = &value["data"];
        println!("{}", value["call"].as_str().unwrap());

        match value["call"].as_str().unwrap() {
            "join" => {
                let username = data["username"].as_str();

                if let None = username {
                    return Message::Unknown;
                }

                let color_str = data["color"].as_str();

                if let None = color_str {
                    return Message::Unknown;
                }

                Message::Join(UserData::new(username.unwrap().to_string(), Color::from(color_str.unwrap())))
            }

            "keypress" => Message::Keypress,
            "finished" => Message::Finished,

            _ => {
                println!("{}", value["call"].as_str().unwrap());
                Message::Unknown
            }
        }
    }
}

/*impl ToString for Message {
    fn to_string(&self) -> String {

    }
}*/
