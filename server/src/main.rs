#![feature(async_closure)]
use server::{input::{nonblocking_stdin, /*input_handler*/}, /*server::Server*/ client::Client};
use tokio::net::TcpListener; 

use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() {
    //let (server_sender, mut server_receiver) = mpsc::channel::<String>(1024);
    
   // let input = nonblocking_stdin();

    print!("[basedtyper]: ");

    let listener = TcpListener::bind("localhost:1337").await.unwrap();

    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            //let clients_clone = clients.clone();

            //let stream = Arc::new(Mutex::new(stream));

            tokio::spawn(async move {
                loop {
                    let mut buf = vec![0 as u8; 1024];

                    //let mut stream_lock = stream.lock().await;
    
                    if let Err(err) = stream.read(&mut buf).await {
                    }

                    //stream_lock.read(&mut buf).await.unwrap();

                    //drop(stream_lock);
                    
                    buf.retain(|byte| byte != &u8::MIN);
            
                    let data = String::from_utf8(buf).unwrap();
    
                    if data.len() > 0 {
                        /*let json: Value = serde_json::from_str(&data).unwrap();
    
                        match json["call"].as_str().unwrap() {
                            "init" => {
                                //let mut clients = clients_clone.lock().await;
                                //let username = &json["data"]["username"].as_str().unwrap();
    
                                /*let client = Client::new(stream.clone(), username.to_string());
                                clients.push(client);*/
    
                                let json = json!({
                                "call": "words",
                                    "data": {
                                        "words": "these are some random words",
                                    }
                                });
        
                                let data = serde_json::to_string(&json).unwrap();
            
                                //sender.send(data).await.unwrap();
                           }    
                            
                           _ => ()
                        }*/
                    }
                }
            });
        }
    }
}
