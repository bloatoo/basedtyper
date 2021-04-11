use basedtyper::{
    event::*,
    app::App,
    handlers::{input_handler, message_handler, connection_handler},
    ui,
};

use std::sync::mpsc::{Receiver, Sender, self};
use serde_json::{json, Value};

use std::{net::TcpStream, cmp::Ordering, io::{self, Read, Write}};
use std::thread;

use crossterm::{ExecutableCommand, cursor::MoveTo, execute, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen}};

use tui::{Terminal, backend::CrosstermBackend, layout::{Alignment, Constraint, Direction, Layout, Margin}, style::{Color, Modifier, Style}, text::{Span, Spans, Text}, widgets::Paragraph};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen).unwrap();

    let (sender, receiver) = mpsc::channel();


    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new(1000);

    let mut app = App::new(terminal.size().unwrap());
    
    println!("\x1b[5 q");

    terminal.clear().unwrap();

    loop {
        if let Ok(_msg) = receiver.try_recv() {
            
        }

        terminal.draw(|f| ui::draw_ui(f, &app)).unwrap();

        if let Ok(Event::Input(event)) = events.next() {
            input_handler(event, &mut app, sender.clone());
        }

        if app.should_exit {
            break;
        }
    }

    app.exit().unwrap();
    Ok(())
}
