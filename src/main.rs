mod event;
mod word;
mod wordlist_parser;
mod app;

use app::{App, State};

use std::{cmp::Ordering, env, io};

use event::*;
use word::Word;

use serde_json::Value;
use termion::{event::Key, raw::IntoRawMode};
use tui::{Terminal, backend::TermionBackend, layout::{Alignment, Constraint, Corner, Direction, Layout, Margin}, style::{Color, Modifier, Style}, text::{Span, Spans, Text}, widgets::Paragraph};

fn usage(args: &[String]) {
    println!(
        "basedtyper

        \rusage:\n \
        \r {} random <word count>            fetches random words and their definitions from APIs
        \r {} wordlist <path to wordlist>    uses a local file as a wordlist
        
        \roptions:\n \
        \r --no-defs                       disable definitions for words
        ",
        &args[0], &args[0]
    )
}

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut app = App::default();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        usage(&args);
        std::process::exit(1);
    }

    let mut words: Vec<Word> = Vec::new();

    match &args[1][..] {
        "random" => {
            let mut word_count = env::args().collect::<Vec<String>>()[2].parse::<u32>();

            if word_count.is_err() {
                word_count = Ok(10)
            }

            let client = reqwest::Client::new();
            let res = client.get(
                &format!(
                    "https://random-word-api.herokuapp.com/word?number={}",
                    word_count.unwrap()
                )[..],
            );

            let text = &res.send().await.unwrap().text().await.unwrap()[..];

            let local_words: Vec<&str> = serde_json::from_str(text).unwrap();

            if args.iter().find(|val| val == &&String::from("--no-defs")).is_none() {
                for word in local_words {
                    let other_res = client
                        .get(
                            &format!(
                                "https://api.dictionaryapi.dev/api/v2/entries/en_US/{}",
                                word
                            )[..],
                        )
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                    let json: Value = serde_json::from_str(&other_res[..]).unwrap();
                    let value = json[0]["meanings"][0]["definitions"][0]["definition"].as_str();

                    if let Some(val) = value {
                        words.push(Word::new(word, val));
                    } else {
                        words.push(Word::new(word, "No definitions found\n"));
                    }
                }
            } else {
                for word in local_words {
                    words.push(Word::new(word, ""));
                }
            }
        }

        "wordlist" => {
            let parsed_words = wordlist_parser::parse(&args[2], &args).await;

            if let Err(err) = parsed_words {
                println!(
                    "\"{}\" is not a valid wordlist: {}",
                    &args[2],
                    err.to_string()
                );

                std::process::exit(1);
            }

            words = parsed_words.unwrap();
        }

        _ => {
            usage(&args);
            std::process::exit(1);
        }
    }

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let words_vec = words
        .iter()
        .map(|elem| elem.get_word().into())
        .collect::<Vec<String>>();

    let word_string = words_vec.join(" ");

    let word_string = word_string.trim_end();

    let events = Events::new();

    let words_split = word_string.split("").collect::<Vec<&str>>();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)])
        .margin(3)
        .split(terminal.size().unwrap());

    print!("{}", termion::clear::All);
    print!("{}", termion::cursor::SteadyBar);

    loop {
        terminal
            .draw(|f| {
                let mut to_be_rendered_str: Vec<Span> = vec![];
                
                match app.state {
                    State::MainMenu => {
                        let mut spans: Vec<Spans> = vec![];

                        for _ in 0..chunks[0].height / 2 - 2 {
                            spans.push(Spans::default());
                        }

                        spans.append(&mut vec![
                            Spans::from(Span::styled("basedtyper", Style::default().fg(Color::Green))),
                            Spans::from(Span::raw("")),
                            Spans::from(Span::raw("t to start typing game")),
                        ]);

                        f.render_widget(Paragraph::new(spans).alignment(Alignment::Center), chunks[0]);
                    }

                    State::EndScreen => {
                        let wpm = (word_string.len() as f64 / 5_f64)
                            / ((app.time_taken as f64 / 1000_f64) / 60_f64);
    
                        let blue = Style::default().fg(Color::Blue);
    
                        let mut spans: Vec<Spans> = vec![];

                        for _ in 0..chunks[0].height / 2 - 3 {
                            spans.push(Spans::default());
                        }

                        spans.append(&mut vec![
                            Spans::from(vec![Span::styled("WPM", blue.add_modifier(Modifier::BOLD)),
                                Span::styled(format!(": {:.2}", wpm), blue)]),

                            Spans::from(vec![Span::styled("Time used", blue.add_modifier(Modifier::BOLD)),
                                Span::styled(format!(": {:.1}s", app.time_taken as f64 / 1000_f64), blue)]),

                            Spans::from(vec![Span::raw("")]),

                            Spans::from(vec![Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                                Span::raw(" to quit")]),

                            Spans::from(vec![Span::styled("r", Style::default().add_modifier(Modifier::BOLD)),
                                Span::raw(" to restart")]),

                            Spans::from(vec![Span::styled("m", Style::default().add_modifier(Modifier::BOLD)),
                                Span::raw(" to go to the main menu")])
                        ]);

                        f.render_widget(
                            Paragraph::new(spans).alignment(Alignment::Center),
                            chunks[0]
                        );
                    }

                    State::TypingGame => {
                        for (index, c) in word_string.split("").enumerate() {
                            if app.input_string.split("").nth(index).is_some() {
                                match index.cmp(&app.current_index) {
                                    Ordering::Less => {
                                        if app.input_string.split("").collect::<Vec<&str>>()[index] != words_split[index] {
                                            to_be_rendered_str.push(Span::styled(c, Style::default().fg(Color::Red)));
    
                                        } else {
                                            to_be_rendered_str.push(Span::styled(
                                                c,
                                                Style::default().fg(Color::DarkGray),
                                            ));
                                        }
                                    }
    
                                    _ => to_be_rendered_str.push(Span::styled(c, Style::default().fg(Color::White))),
                                }

                            } else {
                                to_be_rendered_str.push(Span::styled(c, Style::default()));
                            }
                        }

                        let wpm = (app.input_string.len() as f64 / 5_f64)
                            / (if app.timer.is_some() { (app.timer.unwrap().elapsed().as_millis() as f64 / 1000_f64) / 60_f64 } else { 0_f64 });
    
                        let defs: Vec<&String> =
                            words.iter().map(|elem| elem.get_definition()).collect();
    
                        f.render_widget(
                            Paragraph::new(Text::from(format!("WPM: {:.2}", wpm)))
                                .alignment(Alignment::Center),
                            chunks[0].inner(&Margin {
                                horizontal: 0,
                                vertical: chunks[0].height / 4,
                            }),
                        );
    
                        f.render_widget(
                            Paragraph::new(Spans::from(to_be_rendered_str.clone()))
                                .alignment(Alignment::Center),
                            chunks[0].inner(&Margin {
                                horizontal: 0,
                                vertical: chunks[0].height / 2,
                            }),
                        );
    
                        let index = app.input_string.split(' ').count() - 1;

                        f.render_widget(
                            Paragraph::new(Spans::from(if defs.len() > index {
                                defs[index].clone()
                            } else {
                                String::new()
                            }))
                            .alignment(Alignment::Center),
                            chunks[0].inner(&Margin {
                                horizontal: 0,
                                vertical: chunks[0].height / 3,
                            }),
                        );
    
                        f.set_cursor(
                            chunks[0].x + chunks[0].width / 2 + app.current_index as u16
                                - to_be_rendered_str.len() as u16 / 2,
                            chunks[0].y + chunks[0].height / 2,
                        );
                    }
                _ => (),
                }
            })
            .unwrap();

        if let Ok(Event::Input(event)) = events.next() {
            match event {
                Key::Char(c) => {
                    match app.state {
                        State::EndScreen => {
                            match c {
                                'q' => break,

                                'r' => {
                                    app.restart(State::TypingGame);
                                }

                                'm' => {
                                    app.restart(State::MainMenu);
                                }

                                _ => (),
                            }
                        }

                        State::MainMenu => {
                            match c {
                                't' => {
                                    app.state = State::TypingGame;
                                }

                                _ => ()
                            }
                        }

                        _ => {
                            if app.timer.is_none() {
                                app.start_timer();
                            }

                            if app.current_index <= word_string.len() {
                                app.current_index += 1;
                                app.input_string.push(c);
                            }

                            if word_string == app.input_string {
                                app.state = State::EndScreen;
                                app.time_taken = if app.timer.is_some() { app.timer.unwrap().elapsed().as_millis() } else { 0 };
                            }
                        }
                    }
                }

                Key::Ctrl(c) => {
                    match app.state {
                        State::TypingGame => {
                            match c {
                                'r' => {
                                    app.restart(State::TypingGame);
                                }
                                _ => ()
                            }
                        }

                        _ => ()
                    } 
                }

                Key::Esc => break,

                Key::Backspace => {
                    if app.current_index - 1 > 0 {
                        app.current_index -= 1;
                        app.input_string.pop();
                    }
                }

                _ => (),
            }
        }
    }
    Ok(())
}
