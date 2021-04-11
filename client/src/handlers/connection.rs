use std::io::Read;
use std::sync::mpsc::Sender;
use std::net::TcpStream;

pub fn connection_handler(mut stream: TcpStream, sender: Sender<String>) {
    loop {
        let mut buf = vec![0 as u8; 1024];

        if stream.read(&mut buf).is_err() {
             println!("failed to read from server");
        }

        buf.retain(|byte| byte != &u8::MIN);
        let data = String::from_utf8(buf).unwrap();

        if data.len() > 0 {
            sender.send(data).unwrap();
        }
    }
}
