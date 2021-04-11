use basedtyper::{
    event::*,
    app::App,
    handlers::{message_handler, input_handler},
    ui,
};

use std::{io, sync::mpsc};

use crossterm::{ExecutableCommand, terminal::{enable_raw_mode, EnterAlternateScreen}};

use tui::{Terminal, backend::CrosstermBackend};

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
        if let Ok(msg) = receiver.try_recv() {
            message_handler(msg, &mut app);
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
