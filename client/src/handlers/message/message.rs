use serde_json::json;

pub enum Message {
    Join(UserData),
    Finished(f64),
    Keypress
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

            Self::Keypress => {
                json!({
                    "call": "keypress",
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

pub struct UserData {
    username: String,
    color: String,
}

impl UserData {
    pub fn new(username: String, color: String) -> Self {
        Self {
            username,
            color,
        }
    }
}
