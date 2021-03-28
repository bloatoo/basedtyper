mod event;
mod wordlist_parser;

use std::{io, env, cmp::Ordering, time::Instant};

use event::*;

use serde_json::Value;
use tui::{Terminal, backend::TermionBackend, layout::{Alignment, Constraint, Direction, Layout, Margin}, style::{Color, Modifier, Style}, text::{Span, Spans, Text}, widgets::Paragraph};
use termion::{event::Key, raw::IntoRawMode};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage:\n \
            \r{} random <word count>            fetches random words and their definitions from APIs
            \r{} wordlist <path to wordlist>    uses a local file as a wordlist", &args[0], &args[0]);
        std::process::exit(0);
    }

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let mut words: Vec<(String, String)> = Vec::new();

    match &args[1][..] {
        "random" => {
            let mut word_count = env::args().collect::<Vec<String>>()[2].parse::<u32>();

            if let Err(_) = word_count {
                word_count = Ok(10)
            }

            let client = reqwest::Client::new();
            let res = client.get(&format!("https://random-word-api.herokuapp.com/word?number={}", word_count.unwrap())[..]);
            let text = &res.send().await.unwrap().text().await.unwrap()[..];

            let local_words: Vec<&str> = serde_json::from_str(&text[..]).unwrap();
    
            for word in local_words {
                let other_res = client.get(&format!("https://api.dictionaryapi.dev/api/v2/entries/en_US/{}", word)[..]).send().await.unwrap().text().await.unwrap();

                let json: Value = serde_json::from_str(&other_res[..]).unwrap();
                let value = json[0]["meanings"][0]["definitions"][0]["definition"].as_str();

                if let Some(val) = value {
                    words.push((String::from(word), String::from(val) + "\n"));
                } else {
                    words.push((String::from(word), String::from("No definitions found\n")));
                }
            }

        }
        "wordlist" => {
            let path = &args[2];
            words = wordlist_parser::parse(path).unwrap();
        }
        _ => ()
    }

    let word_string = words.iter().map(|elem| elem.clone().0).collect::<Vec<String>>().join(" ");
    let word_string = word_string.trim_end();

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
                    Span::styled(" - q", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to quit |"),
                    Span::styled(" r", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to restart"),
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
                                let check = input_string.rfind(" ");
                                if let Some(idx) = check {
                                    if index > idx {
                                        to_be_rendered_str.push(Span::styled(c, Style::default().fg(Color::Red)));
                                    } else {
                                        to_be_rendered_str.push(Span::styled(c, Style::default().fg(Color::DarkGray)));
                                    }
                                } else {
                                    to_be_rendered_str.push(Span::styled(c, Style::default().fg(Color::Red)));
                                }
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

                let defs: Vec<&String> = words.iter().map(|elem| &elem.1).collect();

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
                            'r' => {
                                end = false;
                                input_string = String::new();
                                current_index = 1;
                                timer_is_going = false;
                                time_taken = 0;
                                timer = Instant::now();
                            }
                            
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
