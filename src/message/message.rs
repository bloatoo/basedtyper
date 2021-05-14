use serde_json::json;

pub enum Message {
    Join(UserData),
    Finished(f64),
    Keypress(f64)
}

impl ToString for Message {
    fn to_string(&self) -> String {
        let json = match self {
            Self::Join(data) => {
                json!({
                    "call": "join",
                    "data": {
                        "username": data.username,
                        "color": data.color,
                    }
                })
            }

            Self::Keypress(wpm) => {
                json!({
                    "call": "keypress",
                    "data": {
                        "wpm": wpm
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
        };

        json.to_string()
    }
}

#[derive(Debug)]
pub struct UserData {
    pub username: String,
    pub color: String,
    pub wpm: f64
}

impl UserData {
    pub fn new(username: String, color: String, wpm: f64) -> Self {
        Self {
            username,
            color,
            wpm,
        }
    }
}
