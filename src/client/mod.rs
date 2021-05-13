pub mod config;
pub mod app;
pub mod parser;
pub mod ui;
pub mod event;
pub mod handlers;
pub mod io;
pub mod message;

use super::client::{app::App, io::{EventHandler, IOEvent}};

use std::{panic::PanicInfo, sync::mpsc::Receiver};
use std::sync::Arc;
use tokio::sync::Mutex;

use crossterm::{execute, style::Print, terminal::{LeaveAlternateScreen, disable_raw_mode}};

fn panic_hook(info: &PanicInfo<'_>) {
    let location = info.location().unwrap();

    let message = match info.payload().downcast_ref::<&'static str>() {
        Some(msg) => *msg,
        None => match info.payload().downcast_ref::<String>() {
            Some(s) => &s[..],
            None => "Box<Any>"
        }
    };

    disable_raw_mode().unwrap();

    execute!(
        std::io::stdout(),
        LeaveAlternateScreen,
        Print(format!("thread <unnamed> panicked at '{}', {}\n", message, location)),
    ).unwrap();
}

pub async fn start_client() -> Result<(), Box<dyn std::error::Error>> {
    std::panic::set_hook(Box::new(|info| panic_hook(info)));

    let (event_tx, event_rx) = std::sync::mpsc::channel();

    let app = Arc::new(Mutex::new(App::new()));
    let app_clone = app.clone();

    let handler = EventHandler::new(app_clone);

    std::thread::spawn(move || {
        handle_events(handler, event_rx);
    });

    println!("\x1b[5 q");


    /*    if let Ok(msg) = input_receiver.try_recv() {
            let args: Vec<&str> = msg.split(' ')
                .map(|elem| elem.trim())
                .collect();

            match args[0] {
                "connect" => {
                    let host = args[1];
                    let connection = app.connect(host.to_string(), event_tx.clone()).await;

                    if let Ok(conn) = connection {
                        connection_receiver = conn.1;
                        app.connection = Connection::new(conn.0);
                    }
                }

                _ => ()
            }
        }*/

    ui::start(app, event_tx).await.unwrap();

    exit().unwrap();
    Ok(())
}

fn exit() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen, crossterm::cursor::Show)?;
    Ok(())
}

#[tokio::main]
async fn handle_events(mut handler: EventHandler, rx: Receiver<IOEvent>) {
    loop {
        if let Ok(event) = rx.try_recv() {
            handler.handle_event(event).await;
        }
    }
}
