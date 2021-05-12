use std::env;
use basedtyper::{client, server};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match &args[1][..] {
            "serve" | "server" => {
                let port = args
                    .iter()
                    .nth(2);

                match port {
                    Some(port) => {
                        let port = port.parse().unwrap_or(1337_u32);
                        server::start_server(Some(port)).await.unwrap();
                    }
                    None => {
                        server::start_server(None).await.unwrap();
                    }
                }
            }
            _ => ()
        }
    } else {
        client::start_client().await.unwrap();
    }
}
