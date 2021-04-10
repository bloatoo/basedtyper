use basedtyper::{
    event::*,
    app::App,
    input::input_handler,
    ui,
};

use std::sync::mpsc::{Receiver, Sender, self};
use serde_json::{json, Value};

use std::{net::TcpStream, cmp::Ordering, io::{self, Read, Write}};
use std::thread;

use crossterm::{ExecutableCommand, cursor::MoveTo, execute, terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen}};

use tui::{Terminal, backend::CrosstermBackend, layout::{Alignment, Constraint, Direction, Layout, Margin}, style::{Color, Modifier, Style}, text::{Span, Spans, Text}, widgets::Paragraph};

fn handle_connection(mut stream: TcpStream, sender: Sender<String>) {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new(250);

    let mut app = App::new(terminal.size().unwrap());
    
    println!("\x1b[5 q");

    terminal.clear().unwrap();

    loop {
        terminal.draw(|f| ui::draw_ui(f, &app)).unwrap();

        if let Ok(Event::Input(event)) = events.next() {
            input_handler(event, &mut app);
        }

        if app.should_exit {
            break;
        }
    }

    app.exit().unwrap();
    Ok(())
}
