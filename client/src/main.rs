use basedtyper::{app::App, io::EventHandler, ui};

use std::{io, sync::mpsc::Receiver};
use std::sync::Arc;
use tokio::sync::Mutex;

use crossterm::{terminal::{LeaveAlternateScreen, disable_raw_mode}, execute};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    ui::start(app, event_tx.clone()).await.unwrap();

    exit().unwrap();
    Ok(())
}

fn exit() -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, crossterm::cursor::Show)?;
    Ok(())
}

#[tokio::main]
async fn handle_events(mut handler: EventHandler, rx: Receiver<String>) {
    while let Ok(event) = rx.recv() {
        handler.handle_event(event).await;
    }
}
