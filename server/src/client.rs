use tokio::net::TcpStream;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Client {
    pub tcp: Arc<Mutex<TcpStream>>,
    pub username: String,
}

impl Client {
    pub fn new(tcp: Arc<Mutex<TcpStream>>, username: String) -> Self {
        Self {
            tcp,
            username
        }
    }
}
