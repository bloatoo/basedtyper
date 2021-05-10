use server::{client::Client, handlers::input_handler, message::{Message, UserData}, server::Server};
use tokio::net::TcpListener;
use tokio::sync::mpsc::{self, *};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn nonblocking_stdin() -> UnboundedReceiver<String> {
    let (sender, receiver) = mpsc::unbounded_channel();

    std::thread::spawn(move || loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        if let Err(e) = sender.send(buf) {
            println!("{}", e);
        }
    });
    receiver
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { 
    //let (sender, receiver) = mpsc::channel::<String>();
    let mut input = nonblocking_stdin();

    let port = std::env::args().nth(1).unwrap_or(String::from("1337"));
    let port = port.parse::<u32>().unwrap_or(1337);

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    let server = Server::default();

    println!("Server started on port {}.", port);

    let clients = server.clients.clone();

    let mut server_clone = server.clone();

    tokio::spawn(async move {
        loop {
            if let Some(data) = input.recv().await {
                input_handler(data, &mut server_clone).await;
            }
        }
    });

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            //println!("New connection: {}", stream.peer_addr().unwrap());
            let (mut read, mut write) = stream.into_split();

            //let sender = sender.clone();
            let mut server_clone = server.clone();
            let mut server_clone2 = server.clone();

            tokio::spawn(async move {
                let mut buf = vec![0u8; 1024];

                let mut username = String::new();

                if let Err(e) = read.read(&mut buf).await {
                    println!("Failed to read from stream: {}", e.to_string());
                }

                buf.retain(|byte| byte != &u8::MIN);

                if !buf.is_empty() {
                    let message = String::from_utf8(buf).unwrap();

                    if let Message::Join(data) = Message::from(message.clone().as_str()) {
                        username = data.username.clone();

                        let new_user_data = UserData::new(data.username.clone(), data.color.clone());
                        server_clone2.forward(Message::Join(new_user_data).to_json().to_string(), data.username.clone()).await;

                        write.write(server_clone2.create_init_message().await.as_bytes()).await.unwrap();
                        let mut clients_lock = server_clone2.clients.lock().await;
                        clients_lock.push(Client::new(write, username.clone(), data.color.to_string()));
                        println!("New player with username {}", username);

                        drop(clients_lock);
                    }
                }

                loop {
                    let mut buf = vec![0u8; 1024];

                    read.read(&mut buf).await.unwrap();

                    buf.retain(|byte| *byte != u8::MIN);

                    if !buf.is_empty() {
                        server_clone.process_message(String::from_utf8(buf.clone()).unwrap(), username.clone()).await;
                        //server_clone.forward(String::from_utf8(buf).unwrap(), username.clone()).await;
                    }
                }

            });
        }

    }
}
