use std::time::Instant;

use crate::event::Key;
use crate::app::{State, App};
use crossterm::terminal::disable_raw_mode;

pub fn input_handler(key: Key, app: &mut App) {
    match key {
        Key::Up => {
            match app.state {
                State::MainMenu => {
                    app.decrement_index();
                }
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
                _ => ()
            }
        }
        Key::Enter => {
            match app.state {
                State::MainMenu => {
                    match app.current_index {
                        1 => {
                            let words = crate::parser::parse_words("wordlist", Some(String::from("example.basedtyper"))).unwrap();
                            
                            let words_vec = words
                                .iter()
                                .map(|elem| elem.get_word().into())
                                .collect::<Vec<String>>();

                            let mut word_string = words_vec.join(" ");

                            if word_string.len() as u16 > app.chunks[0].width {
                                word_string = String::from(&word_string[..app.chunks[0].width as usize]);
                            }

                            app.words = words;
                            app.word_string = word_string;
                            app.restart(State::TypingGame);
                        }

                        3 => {
                            let words = crate::parser::parse_words("quote", None).unwrap();

                            let words_vec = words
                                .iter()
                                .map(|elem| elem.get_word().into())
                                .collect::<Vec<String>>();

                            let mut word_string = words_vec.join(" ");

                            if word_string.len() as u16 > app.chunks[0].width {
                                word_string = String::from(&word_string[..app.chunks[0].width as usize - 2]);
                            }

                            app.words = words;
                            app.word_string = word_string;
                            app.restart(State::TypingGame);

                        }
                        _ => (),
                    }
                }
                _ => ()
            }
        }

        Key::Char(c) => {
            match app.state {
                State::TypingGame => {
                    if app.timer.is_none() {
                        app.timer = Some(Instant::now());
                    }

                    if app.input_string.len() < app.word_string.len() {
                        app.input_string.push(c);
                        app.increment_index();
                    }

                    if app.input_string == app.word_string {
                        app.state = State::EndScreen;
                        app.time_taken = if app.timer.is_some() { app.timer.unwrap().elapsed().as_millis() } else { 0 };
                    }


                }

                State::EndScreen => {
                    match c {
                        'q' => app.should_exit = true,

                        'r' => {
                            app.restart(State::TypingGame);
                        }

                        'm' => {
                            app.restart(State::MainMenu);
                        }

                        _ => (),
                    }
                }

                _ => ()
            }
        }

        Key::Esc => {
            app.should_exit = true;
        }
        _ => ()
    }
}
