//use crate::server::Server;
use std::{thread};
use std::{io::{Read, Write, stdin}, sync::mpsc::{self, Sender, Receiver}};

pub fn nonblocking_stdin() -> std::sync::mpsc::Receiver<String> {
    let (sender, receiver) = mpsc::channel();

    std::thread::spawn(move || loop {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf).unwrap();
        println!("{}", buf);
        sender.send(buf).unwrap();
    });
    receiver
}

/*pub fn input_handler(msg: String, server: &mut Server) {

}*/
