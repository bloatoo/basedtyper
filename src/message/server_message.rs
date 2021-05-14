use super::UserData;
use serde_json::Value;

#[derive(Debug)]
pub enum ServerMessage {
    Init(Vec<UserData>),
    Join(UserData),
    Keypress((String, f64)),
    Finished(String),
    Start(String),
    Unknown
}

impl From<String> for ServerMessage {
    fn from(data: String) -> Self {
        let json = serde_json::from_str(&data);
        if json.is_err() {
            return Self::Unknown
        }

        let json: Value = json.unwrap();
        let data = &json["data"];

        match json["call"].as_str().unwrap_or("unknown") {
            "join" => {
                let username = data["username"].as_str();
                let color = data["color"].as_str();

                if username.is_none() { return Self::Unknown; }
                if color.is_none() { return Self::Unknown; }

                Self::Join(UserData::new(username.unwrap().to_string(), color.unwrap().to_string(), 0.0))
            }

            "keypress" => {
                let username = data["username"].as_str();
                let wpm = data["wpm"].as_f64();

                if username.is_none() { return Self::Unknown; }
                if wpm.is_none() { return Self::Unknown; }

                Self::Keypress((username.unwrap().to_string(), wpm.unwrap()))
            }

            "init" => {
                let players = data["players"].as_array().unwrap();

                let players: Vec<UserData> = players.iter().map(|player| {
                    let username = player["username"].as_str().unwrap();
                    let color = player["color"].as_str().unwrap();
                    //let wpm = player["wpm"].as_f64().unwrap();

                    UserData::new(username.to_string(), color.to_string(), 0.0)
                }).collect();

                Self::Init(players)
            }

            "finished" => {
                let username = data["username"].as_str();
                if username.is_none() { return Self::Unknown; }

                Self::Finished(username.unwrap().to_string())
            }
            "start" => {
                let words = data["words"].as_str().unwrap();
                Self::Start(words.to_string())
            }

            _ => panic!("invalid message: {}", data)
        }
    }
}
