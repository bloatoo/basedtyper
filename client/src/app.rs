use io::{Read, Write};
use std::sync::Arc;
use tui::layout::{Constraint, Direction, Layout, Rect};
use crossterm::{execute, terminal::{LeaveAlternateScreen, disable_raw_mode}};

use crate::{handlers::message::{Message, UserData}, ui::wordlist::Wordlist};

use super::config::Config;
use std::{io, net::TcpStream, path::Path, sync::{Mutex, mpsc::{self, Receiver}}, time::Instant};

pub struct App {
    pub state: State,
    pub config: Config,
    pub input_string: String,
    pub time_taken: u128,
    pub timer: Option<Instant>,
    pub connection: Option<Arc<Mutex<TcpStream>>>,
    pub current_index: usize,
    pub current_error: String,
    pub should_exit: bool,
    pub wordlist: Wordlist,
    pub chunks: Vec<Rect>,
}

pub enum State {
    MainMenu,
    MultiplayerEndScreen,
    EndScreen,
    Waiting,
    TypingGame,
    WordlistPrompt,
    HostPrompt,
}

impl App {
    pub fn new(area: Rect) -> Self {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let config = Config::new();

        let (config, err) = if config.is_err() {
            (Config::default(), config.err().unwrap().to_string())
        } else { 
            (config.unwrap(), String::new()) 
        };

        Self {
            state: State::MainMenu,
            input_string: String::new(),
            timer: None,
            time_taken: 0,
            current_index: 1,
            config,
            current_error: err,
            connection: None,
            should_exit: false,
            wordlist: Wordlist::new(Vec::new()),
            chunks,
        }
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
    
    pub fn close_connection(&mut self) {
        let conn = self.connection.clone().unwrap();
        let conn_lock = conn.lock().unwrap();
        conn_lock.shutdown(std::net::Shutdown::Both).unwrap();
        drop(conn_lock);
        self.connection = None;
    }

    pub fn connect(&mut self, host: String) -> Result<(TcpStream, Receiver<String>), std::io::Error> {
        let stream = TcpStream::connect(host);

        if let Err(e) = stream {
            self.current_error = e.to_string();
            return Err(e);
        }

        let (connection_sender, connection_receiver) = mpsc::channel::<String>();

        let mut stream = stream.unwrap();
        let stream_clone = stream.try_clone().unwrap();
        self.state = State::Waiting;

        let username = self.config.multiplayer.username.clone();
        let color = self.config.multiplayer.color.clone();

        let join_message = Message::Join(UserData::new(username, color));

        stream.write(join_message.to_string().as_bytes()).unwrap();

        std::thread::spawn(move || loop {
            let mut buf = vec![0u8; 1024];

            if stream.read(&mut buf).is_err() {
                break;
            }

            buf.retain(|byte| byte != &u8::MIN);

            if !buf.is_empty() {
                let data = String::from_utf8(buf).unwrap();
                connection_sender.send(data).unwrap();
            }
        });

        Ok((stream_clone, connection_receiver))
    }

    /*pub fn send_conn(&mut self, data: String) {
        if let Some(conn) = self.connection.clone() {
            conn.send(data).unwrap();
        }
    }*/
    
    pub fn restart(&mut self, state: State) {
        self.input_string = String::new();
        self.current_index = 1;
        self.time_taken = 0;
        self.current_error = String::new();

        match state {
            State::TypingGame => {
                self.timer = Some(Instant::now());
            }
            _ => self.timer = None
        }

        self.state = state;
    }

    pub fn start_timer(&mut self) {
        self.timer = Some(Instant::now());
    }

    pub fn exit(&self) -> Result<(), Box<dyn std::error::Error>> {
        disable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, LeaveAlternateScreen, crossterm::cursor::Show)?;
        Ok(())
    }

    pub fn locate_wordlist(&self) -> String {
        let wordlist_name = if self.input_string.ends_with(".basedtyper") {
            self.input_string.to_string()
        } else {
            self.input_string.clone() + ".basedtyper"
        };

        let path_str = &(self.config.general.wordlist_directory.clone() + "/" + &wordlist_name);
        let path = Path::new(path_str);

        let path = if path.is_file() {
            path.to_str().unwrap().to_string()
        } else {
            wordlist_name
        };

        path
    }

    pub fn decrement_index(&mut self) {
        if self.current_index - 1 > 0 {
            self.current_index -= 1;
        }
    }

    pub fn increment_index(&mut self) {
        self.current_index += 1;
    }
}
