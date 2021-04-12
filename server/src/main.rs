use tokio::net::{TcpListener};
use tokio::io::AsyncReadExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let listener = TcpListener::bind("localhost:1337").await.unwrap();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        println!("new connection");

        tokio::spawn(async move {
            loop {
                let mut buf = String::new();

                if let Err(err) = stream.read_to_string(&mut buf).await {
                    println!("{}", err.to_string());
                }

                if !buf.is_empty() {
                    println!("{}", buf);
                }
            }
        });
    }
}

