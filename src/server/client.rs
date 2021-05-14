use tokio::net::tcp::OwnedWriteHalf;

pub struct Client {
    pub writer: OwnedWriteHalf,
    pub finished: bool,
    pub input_correct: bool,
    pub input_len: u16,
    pub username: String,
    pub color: String,
    pub wpm: f64,
}

impl Client {
    pub fn new(writer: OwnedWriteHalf, username: String, color: String) -> Self {
        Self {
            writer,
            color,
            finished: false,
            input_correct: true,
            input_len: 0,
            username,
            wpm: 0.0,
        }
    }
}
