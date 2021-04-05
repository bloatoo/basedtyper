use std::{
    sync::{
        Arc, Mutex
    },
    net::{
        TcpListener, TcpStream
    },
    io::{Read, Write}
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    let port = if args.len() > 1 {
        args[1].parse::<u32>().expect("failed to parse port from argument")
    } else {
        1337
    };

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("failed to bind");

    listener.set_nonblocking(true).expect("failed");

    println!("server started on port {}", port);

    for stream in listener.incoming() {
        let mut clients = clients.lock().unwrap();

        match stream {
            Ok(mut stream) => {
                println!("new connection: {}", stream.peer_addr().unwrap());
                clients.push(stream.try_clone().unwrap());
                std::thread::spawn(move || loop {
                    let mut buf = vec![0 as u8; 1024];

                    if stream.read(&mut buf).is_err() {
                        
                    }
                    
                    buf.retain(|byte| byte != &u8::MIN);
                    let data = String::from_utf8(buf).unwrap();
                    if data.len() > 0 {
                        println!("{}", data);
                        //let json = serde_json::from_str(data);
                    }
                });
            }

            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            }

            Err(err) => {
                eprintln!("error: {}", err);
            }
        }
    }
}
