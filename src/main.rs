mod event;

use event::*;
use std::io;
use std::cmp::Ordering;

use tui::{Terminal, backend::TermionBackend, layout::{Alignment, Constraint, Direction, Layout, Margin}, style::{Color, Modifier, Style}, text::{Span, Spans}, widgets::Paragraph};
use termion::{event::Key, raw::IntoRawMode};
use std::time::Instant;
use std::env;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut word_count = env::args().collect::<Vec<String>>()[1].parse::<u32>();

    if let Err(_) = word_count {
        word_count = Ok(10)
    }
    let events = Events::new();
    let client = reqwest::Client::new();

    let mut current_index: usize = 1;
    let mut input_string: String = String::new();
    let mut end = false;
    let mut time_taken: u128 = 0;

    let res = client.get(&format!("https://random-word-api.herokuapp.com/word?number={}", word_count.unwrap())[..]).send().await.unwrap();

    let text = res.text().await.unwrap();
    let words: Vec<&str> = serde_json::from_str(&text[..]).unwrap();


    let word_string = words.join(" ");

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)])
        .margin(1)
        .split(terminal.size().unwrap());

    print!("{}", termion::clear::All);
    print!("{}", termion::cursor::SteadyBar);

    let now = Instant::now();
   
    loop {
        terminal.draw(|f| {
            let mut to_be_rendered_str: Vec<Span> = vec![];

            if end {
                let wpm = (word_string.len() as f64 / 5 as f64) / ((time_taken as f64 / 1000 as f64) / 60 as f64);

                let blue = Style::default().fg(Color::Blue);

                let spans: Vec<Span> = vec![
                    Span::styled("WPM", blue.add_modifier(Modifier::BOLD)), 
                    Span::styled(format!(": {:.2} | ", wpm), blue),
                    Span::styled("Time used", blue.add_modifier(Modifier::BOLD)),
                    Span::styled(format!(": {:.1}s\n\n", time_taken as f64 / 1000 as f64), blue),
                    Span::styled(" - Q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to quit"),
                ];
                
                f.render_widget(Paragraph::new(Spans::from(spans)).alignment(Alignment::Center), chunks[0].inner(&Margin { horizontal: 0, vertical: chunks[0].height / 2 }));
            } else {
                for (index, c) in word_string.split("").enumerate() {
                    match index.cmp(&current_index) {
                        Ordering::Equal => {
                            to_be_rendered_str.push(Span::styled(c, Style::default()));
                        },

                        Ordering::Less => {
                            if input_string[..current_index - 1] != word_string[..current_index - 1] {
                                to_be_rendered_str.push(Span::styled(c, Style::default().fg(Color::Red)));
                            } else {
                                to_be_rendered_str.push(Span::styled(c, Style::default().fg(Color::DarkGray)));
                            }
                        }
    
                        _ => {
                            to_be_rendered_str.push(Span::styled(c, Style::default()));
                        }
                    }
                }

                f.render_widget(Paragraph::new(Spans::from(to_be_rendered_str.clone())).alignment(Alignment::Center), chunks[0].inner(&Margin { horizontal: 0, vertical: chunks[0].height / 2 }));

                f.set_cursor(chunks[0].x + chunks[0].width / 2 + current_index as u16 - to_be_rendered_str.len() as u16 / 2, chunks[0].y + chunks[0].height / 2);
            }
        }).unwrap();

        if let Ok(Event::Input(event)) = events.next() {
            match event {
                Key::Char(c) => {
                    if !end {
                        if current_index <= word_string.len() {
                            current_index += 1;
                            input_string.push(c);
                        }
                        if word_string == input_string {
                            end = true;
                            time_taken = now.elapsed().as_millis();
                        }
                    } else {
                        match c {
                            'q' => break,
                            _ => ()
                        }
                    }
                }   

                Key::Esc => break,

                Key::Backspace => {
                    if current_index - 1 > 0 {
                        current_index -= 1;
                        input_string.pop();
                    }
                }

                _ => ()
            }            
        }
    }
    Ok(())
}
