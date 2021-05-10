use std::io::Write;
use std::{sync::mpsc::Sender, time::Instant};
use crate::{event::Key, parser::Word, ui::wordlist::Wordlist};
use crate::{parser, app::{State, App}};

use super::message::Message;

fn set_wordlist(mode: &str, wordlist_path: Option<String>, app: &mut App) {
    let words = parser::parse_words(mode.to_string().as_str(), wordlist_path);

    if let Err(err) = words {
        app.current_error = err.to_string();
        return;
    }

    let words = words.unwrap();
    
    let mut words_string = words
        .iter()
        .map(|word| word.get_word().clone())
        .collect::<Vec<String>>()
        .join(" ");

    if words_string.len() as u16 > app.chunks[0].width {
        words_string = String::from(&words_string[..app.chunks[0].width as usize]);
    }

    let defs = words
        .iter()
        .map(|word| word.get_definition().clone())
        .collect::<Vec<String>>();

    let words: Vec<Word> = words_string
        .split(" ")
        .enumerate()
        .map(|(index, item)| Word::new(item, &defs[index][..]))
        .collect();

    let wordlist = Wordlist::from(words);
 
    app.wordlist = wordlist;
    app.restart(State::TypingGame);
}

pub fn input_handler(key: Key, app: &mut App, sender: Sender<String>, _conn_sender: Sender<String>) {
    match key {
        Key::Up => {
            match app.state {
                State::MainMenu => app.decrement_index(),
                _ => ()
            }
        }

        Key::Down => {
            match app.state {
                State::MainMenu => {
                    if app.current_index < 3 {
                        app.increment_index();
                    }
                }

                _ => ()
            }
        }

        Key::Backspace => {
            match app.state {
                State::TypingGame => {
                    app.input_string.pop();
                    app.decrement_index();
                }

                State::WordlistPrompt | State::HostPrompt => {
                    app.input_string.pop();
                },

                _ => ()
            }
        }

        Key::Enter => {
            match app.state {
                State::WordlistPrompt => set_wordlist("wordlist", Some(app.locate_wordlist()), app),
                State::HostPrompt => sender.send(format!("connect {}", app.input_string)).unwrap(),
                State::MainMenu => {
                    match app.current_index {
                        1 => app.restart(State::WordlistPrompt),
                        2 => app.restart(State::HostPrompt),
                        3 => set_wordlist("quote", None, app),
                        _ => (),
                    }
                }
                _ => ()
            }
        }

        Key::Ctrl(c) => {
            match app.state {
                State::TypingGame => {
                    match c {
                        'r' => {
                            match app.connection.enabled {
                                true => (),
                                false => app.restart(State::TypingGame),
                            }
                        }
                        'c' => {
                            match app.connection.enabled {
                                true => {
                                    app.close_connection();
                                    app.restart(State::MainMenu);
                                }

                                false => app.restart(State::MainMenu), 
                            }
                            
                        }
                        _ => (),
                    }
                }

                _ => (),
            }
        }

        Key::Char(c) => {
            match app.state {
                State::TypingGame => {
                    if app.timer.is_none() {
                        app.timer = Some(Instant::now());
                    }

                    let word_string = app.wordlist.to_string();

                    if app.input_string.len() < word_string.len() {
                        app.input_string.push(c);

                        if app.connection.enabled {
                            let sock = app.connection.tcp.clone().unwrap();
                            let mut sock_lock = sock.lock().unwrap();

                            let message = Message::Keypress.to_string();

                            if app.input_string.trim() != word_string {
                                sock_lock.write(message.as_bytes()).unwrap();
                            }

                            drop(sock_lock);
                        }

                        app.increment_index();
                    }

                    if app.input_string.trim() == word_string {
                        app.time_taken = if app.timer.is_some() { app.timer.unwrap().elapsed().as_millis() } else { 0 };
                        match app.connection.enabled {
                            true => {
                                app.state = State::MultiplayerEndScreen;

                                let sock = app.connection.tcp.clone().unwrap();
                                let mut sock_lock = sock.lock().unwrap();

                                let wpm = (app.wordlist.to_string().len() as f64 / 5_f64)
                                    / ((app.time_taken as f64 / 1000_f64) / 60_f64);

                                let message = Message::Finished(wpm).to_string();

                                sock_lock.write(message.as_bytes()).unwrap();
                            }

                            false => app.state = State::EndScreen,
                        }
                    }
                }

                State::WordlistPrompt | State::HostPrompt => app.input_string.push(c),

                State::MultiplayerEndScreen => {
                    match c {
                        'q' => {
                            app.close_connection();
                            app.restart(State::MainMenu);
                        }
                        _ => ()
                    }
                }

                State::EndScreen => {
                    match c {
                        'q' => app.should_exit = true,
                        'r' => app.restart(State::TypingGame),
                        'm' => app.restart(State::MainMenu),
                        _ => (),
                    }
                }
                _ => ()
            }
        }

        Key::Esc => {
            match app.state {
                State::MainMenu => app.should_exit = true,
                _ => app.restart(State::MainMenu),
            }
        }

        _ => ()
    }
}
