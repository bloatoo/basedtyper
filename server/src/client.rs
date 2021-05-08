use tokio::net::tcp::OwnedWriteHalf;

pub struct Client {
    pub writer: OwnedWriteHalf,
    pub username: String,
}

impl Client {
    pub fn new(writer: OwnedWriteHalf, username: String) -> Self {
        Self {
            writer,
            username
        }
    }
}
