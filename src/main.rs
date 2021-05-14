use basedtyper::{client::start_client, server::start_server, utils};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();

    match args.nth(1) {
        Some(val) => {
            match val.as_str() {
                "serve" => {
                    let port: u32 = match args.next() {
                        Some(arg) => arg.trim().parse().unwrap_or(1337),
                        None => 1337
                    };

                    start_server(port).await.unwrap();
                }

                "generate" => {
                    match args.next() {
                        Some(path) => utils::generate_wordlist(path).await.unwrap(),
                        None => println!("No file path was given."),
                    }
                }

                _ => ()
            }
        }

        None => {
            start_client().await.unwrap();
        }
    }
    Ok(())
}
