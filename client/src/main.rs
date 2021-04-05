use basedtyper::{
    event::*,
    app::{App, State},
    parser,
};
//use mpsc::{Receiver, Sender, self};

use std::{/*net::TcpStream,*/ cmp::Ordering, io};//{self, Read, Write}};
//use std::thread;

use termion::{event::Key, raw::IntoRawMode};
use tui::{Terminal, backend::TermionBackend, layout::{Alignment, Constraint, Direction, Layout, Margin}, style::{Color, Modifier, Style}, text::{Span, Spans, Text}, widgets::Paragraph};

/*fn handle_connection(mut stream: TcpStream, sender: Sender<String>) {
    loop {
        let mut buf = vec![0 as u8; 1024];

        if stream.read(&mut buf).is_err() {
             println!("failed");
        }

        buf.retain(|byte| byte != &u8::MIN);
        let data = String::from_utf8(buf).unwrap();

        if data.len() > 0 {
            sender.send(data).unwrap();
        }
    }
}*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::default();

    //let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let events = Events::new();

    print!("{}", termion::clear::All);
    print!("{}", termion::cursor::SteadyBar);

    loop {
        let words_vec = app.words
            .iter()
            .map(|elem| elem.get_word().into())
            .collect::<Vec<String>>();

        /*if let Ok(val) = receiver.try_recv() {
            println!("{}", val);
            std::process::exit(0);
        }*/

        let word_string = words_vec.join(" ");

        let mut word_string = word_string.trim_end();

        let words_split = word_string.split("").collect::<Vec<&str>>();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Percentage(100)])
            .split(terminal.size().unwrap());

        if word_string.len() as u16 > chunks[0].width {
            word_string = &word_string[..chunks[0].width as usize - 2];
        }
    
        let re = regex::Regex::new("^\t\r\n\x20-\x7E]+").unwrap();

        let word_string = re.replace_all(word_string, " ");
        let word_string = word_string.trim();

        terminal.draw(|f| {
            let mut to_be_rendered_str: Vec<Span> = vec![];
            
            match app.state {
                State::MainMenu => {
                    let mut spans: Vec<Spans> = vec![];

                    for _ in 0..chunks[0].height / 2 - 2 {
                        spans.push(Spans::default());
                    }

                    spans.push(Spans::from(Span::raw("")));

                    let menu = vec![
                        String::from("     wordlist     "),
                        String::from(" quote (UNSTABLE) ")
                    ];

                    for (index, elem) in menu.iter().enumerate() {
                        if app.current_index - 1 == index {
                            spans.push(Spans::from(Span::styled(elem, Style::default().fg(Color::Green))));
                        } else {
                            spans.push(Spans::from(Span::raw(elem)));
                        }
                    }

                    spans[chunks[0].height as usize / 2 - 5] = Spans::from(Span::styled("basedtyper", Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)));

                    for _ in 0..(chunks[0].height / 3) / 2 {
                        spans.push(Spans::default());
                    }

                    if app.wordlist.0 {
                        spans.push(Spans::from(Span::raw(format!("wordlist name: {}", app.wordlist.1))));
                    }

                    for _ in 0..(chunks[0].height / 3) / 2 {
                        spans.push(Spans::default());
                    }

                    spans.push(Spans::from(Span::raw(format!("wordlist directory: {}", &app.config.wordlist_directory))));

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
                                    if app.input_string.split("").collect::<Vec<&str>>()[index]!= words_split[index] 
                                        && word_string[..app.input_string.len() - 1].trim_start() != app.input_string.trim_start() 
                                    {
                                        to_be_rendered_str.push(Span::styled(c, Style::default().bg(Color::Red)));
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
                        app.words.iter().map(|elem| elem.get_definition()).collect();

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
                                '\n' => {
                                    match app.current_index {
                                        1 => {
                                            if app.wordlist.0 {
                                                let parsed = parser::parse_words("wordlist", &mut app);

                                                if parsed.is_ok() {
                                                    app.restart(State::TypingGame);
                                                    app.wordlist = (false, String::new());
                                                }
                                            } else {
                                                app.wordlist.0 = true;
                                            }
                                        }

                                        2 => {
                                            parser::parse_words("quote", &mut app).unwrap();
                                            app.restart(State::TypingGame);

                                            /*let mut stream = TcpStream::connect("localhost:1337").unwrap();
                                            stream.write(b"{ \"username\": \"bloatoo\" }").unwrap();
                                            let sender = sender.clone();

                                            thread::spawn(move || handle_connection(stream, sender));*/
                                            //testing
                                        }

                                        _ => ()
                                    }

                                }

                                _ => {
                                    if app.wordlist.0 {
                                        app.wordlist.1.push(c);
                                    }
                                }
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

                            if word_string.trim() == app.input_string {
                                app.state = State::EndScreen;
                                app.time_taken = if app.timer.is_some() { app.timer.unwrap().elapsed().as_millis() } else { 0 };
                            }
                        }
                    }
                }

                Key::Up => {
                    match app.state {
                        State::MainMenu => {
                            if app.current_index + 1 > 2  {
                                app.decrement_index()
                            }
                        }
                        _ => ()
                    }
                }

                Key::Down => {
                    match app.state {
                        State::MainMenu => {
                            app.increment_index()
                        }

                        _ => ()
                    }
                }

                Key::Ctrl(c) => {
                    match app.state {
                        State::TypingGame => {
                            match c {
                                'r' => {
                                    app.restart(State::TypingGame);
                                }

                                'c' => {
                                    app.restart(State::MainMenu);
                                }

                                _ => ()
                            }
                        }

                        _ => ()
                    } 
                }

                Key::Esc => break,

                Key::Backspace => {
                    match app.state {
                        State::TypingGame => {
                            if app.current_index - 1 > 0 {
                                app.decrement_index();
                                app.input_string.pop();
                            }
                        }

                        State::MainMenu => {
                            app.wordlist.1.pop();
                        }

                        _ => ()
                    }
                }

                _ => (),
            }
        }
    }
    Ok(())
}
