use std::net::TcpListener;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    let port = if args.len() > 1 {
        args[1].parse::<u32>().expect("failed to parse port from argument")
    } else {
        1337
    };

    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).expect("failed to bind");

    println!("server started on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new connection: {}", stream.peer_addr().unwrap());
            }

            Err(err) => {
                eprintln!("error: {}", err);
            }
        }
    }
}
