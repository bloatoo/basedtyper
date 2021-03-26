mod event;

use event::*;
use std::io;
use std::cmp::Ordering;

use tui::{Terminal, backend::TermionBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::{Span, Spans}, widgets::Paragraph};
use termion::{event::Key, raw::IntoRawMode};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();

    let mut current_index: usize = 1;
    let test_str: String = String::from("This is a test");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .margin(1)
        .split(terminal.size().unwrap());

    print!("{}", termion::clear::All);
    print!("{}", termion::cursor::SteadyBar);
   
    loop {
        terminal.draw(|f| {
            let mut to_be_rendered_str: Vec<Span> = vec![];

            for (index, c) in test_str.split("").enumerate() {
                match index.cmp(&current_index) {
                    Ordering::Equal => {
                        to_be_rendered_str.push(Span::styled(c, Style::default()));
                    },

                    Ordering::Less => {
                        to_be_rendered_str.push(Span::styled(c, Style::default().fg(Color::DarkGray)));
                    }

                    _ => {
                        to_be_rendered_str.push(Span::styled(c, Style::default()));
                    }
                }
            }

            f.render_widget(Paragraph::new(Spans::from(to_be_rendered_str.clone())), chunks[0]);
            f.set_cursor(chunks[0].x + current_index as u16 - 1, chunks[0].y);
        }).unwrap();

        if let Ok(Event::Input(event)) = events.next() {
            match event {
                Key::Char(c) => {
                    match c {
                        'q' => break,

                        _ => {
                            current_index += 1;
                        }
                    }
                }

                Key::Backspace => {
                    if current_index - 1 > 0 {
                        current_index -= 1;
                    }
                }

                _ => ()
            }            
        }
    }
    Ok(())
}
