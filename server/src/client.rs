use std::net::TcpStream;

pub struct Client {
    pub tcp: TcpStream,
    pub username: String,
}

impl Client {
    pub fn new(tcp: TcpStream, username: String) -> Self {
        Self {
            tcp,
            username
        }
    }
}
