use std::time::Instant;

use crate::event::Key;
use crate::{parser, app::{State, App}};

fn set_wordlist(mode: &str, wordlist_path: Option<String>, app: &mut App) {
    let words = parser::parse_words(mode.to_string().as_str(), wordlist_path).unwrap();
    
    let words_vec = words
        .iter()
        .map(|elem| (*elem).get_word().into())
        .collect::<Vec<String>>();
    
    let mut word_string = words_vec.join(" ");
   
    if word_string.len() as u16 > app.chunks[0].width {
        word_string = String::from(&word_string[..app.chunks[0].width as usize]);
    }
 
    app.words = words;
    app.word_string = word_string;
    app.restart(State::TypingGame);
}

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
                            match app.wordlist.0 {
                                false => app.wordlist.0 = true,
                                true => set_wordlist("wordlist", Some(app.locate_wordlist()), app),
                            }
                        }

                        3 => set_wordlist("quote", None, app),

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

                State::MainMenu => {
                    if app.wordlist.0 {
                        app.wordlist.1.push(c);
                    } else if app.host.0 {
                        app.host.1.push(c);
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
