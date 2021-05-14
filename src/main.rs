use basedtyper::{app::App, io::{EventHandler, IOEvent}, ui, wordlist_generator};
use basedtyper::server::start_server;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();

    match args.next() {
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
                        Some(path) => wordlist_generator::generate_wordlist(path).await.unwrap(),
                        None => println!("No file path was given."),
                    }
                }

                _ => ()
            }
        }

        None => {
            std::panic::set_hook(Box::new(|info| panic_hook(info)));

            let (event_tx, event_rx) = std::sync::mpsc::channel();

            let app = Arc::new(Mutex::new(App::default()));
            let app_clone = app.clone();
    
            let handler = EventHandler::new(app_clone);
    
            std::thread::spawn(move || {
                handle_events(handler, event_rx);
            });

            println!("\x1b[5 q");
    
            ui::start(app, event_tx).await.unwrap();
    
            exit().unwrap();
        }
    }
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
        if let Ok(ev) = rx.try_recv() {
            handler.handle_event(ev).await;
        }
    }
}
