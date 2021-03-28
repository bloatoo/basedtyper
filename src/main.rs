#![feature(async_closure)]
mod event;

use event::*;
use serde_json::Value;
use std::{io, sync::mpsc, thread};
use std::cmp::Ordering;

use tui::{Terminal, backend::TermionBackend, layout::{Alignment, Constraint, Direction, Layout, Margin}, style::{Color, Modifier, Style}, text::{Span, Spans, Text}, widgets::Paragraph};
use termion::{event::Key, raw::IntoRawMode};
use std::time::Instant;
use std::env;

// use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut word_count = env::args().collect::<Vec<String>>()[1].parse::<u32>();

    if let Err(_) = word_count {
        word_count = Ok(10)
    }

    let mut definition_string = String::new();

    let client = reqwest::Client::new();

    let res = client.get(&format!("https://random-word-api.herokuapp.com/word?number={}", word_count.unwrap())[..]);
    let text = &res.send().await.unwrap().text().await.unwrap()[..];

    let words: Vec<&str> = serde_json::from_str(&text[..]).unwrap();

    let word_string = words.join(" ");

    for word in words {
        let other_res = client.get(&format!("https://api.dictionaryapi.dev/api/v2/entries/en_US/{}", word)[..]).send().await.unwrap().text().await.unwrap();

        let json: Value = serde_json::from_str(&other_res[..]).unwrap();
        let value = json[0]["meanings"][0]["definitions"][0]["definition"].as_str();

        if let Some(val) = value {
            definition_string.push_str(&(String::from(val) + "\n")[..]);
        } else {
            definition_string.push_str("No definitions found\n");
        }
    }

    let events = Events::new();

    let mut current_index: usize = 1;
    let mut input_string: String = String::new();
    let mut end = false;
    let mut time_taken: u128 = 0;
    let mut timer: Instant = Instant::now();
    let mut timer_is_going = false;

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

            if end {
                let wpm = (word_string.len() as f64 / 5f64) / ((time_taken as f64 / 1000f64) / 60f64);

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

                let wpm = (input_string.len() as f64 / 5 as f64) / ((timer.elapsed().as_millis() as f64 / 1000 as f64) / 60 as f64);

                let defs: Vec<&str> = definition_string.split("\n").collect();

                f.render_widget(Paragraph::new(Text::from(format!("WPM: {:.2}", wpm))).alignment(Alignment::Center), chunks[0].inner(&Margin { horizontal: 0, vertical: chunks[0].height / 4 }));
                f.render_widget(Paragraph::new(Spans::from(to_be_rendered_str.clone())).alignment(Alignment::Center), chunks[0].inner(&Margin { horizontal: 0, vertical: chunks[0].height / 2 }));
                f.render_widget(Paragraph::new(Spans::from(defs[input_string.split(" ").collect::<Vec<&str>>().len() - 1].clone())).alignment(Alignment::Center), chunks[0].inner(&Margin { horizontal: 0, vertical: chunks[0].height / 3 }));

                f.set_cursor(chunks[0].x + chunks[0].width / 2 + current_index as u16 - to_be_rendered_str.len() as u16 / 2, chunks[0].y + chunks[0].height / 2);
            }
        }).unwrap();

        if let Ok(Event::Input(event)) = events.next() {
            match event {
                Key::Char(c) => {
                    if !end {
                        if !timer_is_going {
                            timer_is_going = true;
                            timer = Instant::now();
                        }
                        if current_index <= word_string.len() {
                            current_index += 1;
                            input_string.push(c);
                        }
                        if word_string == input_string {
                            end = true;
                            time_taken = timer.elapsed().as_millis();
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
