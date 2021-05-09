pub mod user_data;
use serde_json::{Value, json};
pub use user_data::{UserData, Color};

pub trait Forwardable {
    fn forwardable(&self, username: String) -> String;
}

pub enum Message {
    Join(UserData),
    Keypress/*(String)*/,
    Finished(f64),
    Unknown,
}

impl<'a> From<&'a str> for Message {
    fn from(data: &'a str) -> Self {
        let value = serde_json::from_str(&data.to_string());

        if let Err(_) = value {
            return Message::Unknown;
        }

        let value: Value = value.unwrap();

        let data = &value["data"];

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
            "finished" => {
                let wpm = data["wpm"].as_f64();
                if let None = wpm {
                    return Message::Unknown;
                }

                Message::Finished(wpm.unwrap())
            }

            _ => {
                println!("{}", value["call"].as_str().unwrap());
                Message::Unknown
            }
        }
    }
}

impl Message {
    pub fn to_json(&self) -> Value {
        match self {
            Self::Join(data) => {
                json!({
                    "call": "join",
                    "data": {
                        "username": data.username,
                        "color": data.color.to_string(),
                    }
                })
            }

            Self::Finished(wpm) => {
                json!({
                    "call": "finished",
                    "data": {
                        "wpm": wpm,
                    }
                })
            }
            Self::Keypress => {
                json!({
                    "call": "keypress",
                })
            }
            Self::Unknown => json!({})
        }
    }

}

impl Forwardable for Message {
    fn forwardable(&self, username: String) -> String {
        match self {
            Self::Keypress => {
                json!({
                    "call": "keypress",
                    "data": {
                        "username": username,
                    }
                })
            }

            Self::Finished(wpm) => {
                json!({
                    "call": "finished",
                    "data": {
                        "username": username,
                        "wpm": wpm,
                    }
                })
            }
            Self::Unknown => json!({}),
            Self::Join(_) => self.to_json(),
        }.to_string()
    }
}
