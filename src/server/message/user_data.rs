use serde_json::json;

#[derive(Clone)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Unknown,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Self::Red => "red",
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Blue => "blue",
            Self::Magenta => "magenta",
            Self::Cyan => "cyan",
            Self::Unknown => "white"
        }.to_string()
    }
}

impl<'a> From<&'a str> for Color {
    fn from(data: &'a str) -> Self {
        match data.to_string().as_str() {
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            _ => Color::Unknown,
        }
    }
}

pub struct UserData {
    pub username: String,
    pub color: Color,
    pub wpm: f64,
}

impl UserData {
    pub fn new(username: String, color: Color, wpm: f64) -> Self {
        Self { username, color, wpm }
    }
}

impl ToString for UserData {
    fn to_string(&self) -> String {
        json!({
            "username": self.username,
            "color": self.color.to_string(),
        }).to_string()
    }
}
