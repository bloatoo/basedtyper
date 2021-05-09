pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Unknown,
}

impl<T: ToString> From<T> for Color {
    fn from(data: T) -> Self {
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
}

impl UserData {
    pub fn new(username: String, color: Color) -> Self {
        Self { username, color }
    }
}
