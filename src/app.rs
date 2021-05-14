use std::sync::Arc;
use tui::layout::Rect;
use std::sync::mpsc::Sender;

use super::{message::{Message, UserData}, io::IOEvent, ui::wordlist::Wordlist};

use tokio::net::{TcpStream, tcp::OwnedWriteHalf};
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::config::Config;

use std::{
    path::Path,
    time::Instant
};

#[derive(Clone)]
pub struct Connection {
    pub tcp: Option<Arc<Mutex<OwnedWriteHalf>>>,
    pub enabled: bool,
    pub players: Vec<Player>
}

impl Connection {
    pub fn new(stream: OwnedWriteHalf) -> Self {
        Self {
            tcp: Some(Arc::new(Mutex::new(stream))),
            enabled: false,
            players: vec![],
        }
    }

    pub fn none() -> Self {
        Self {
            tcp: None,
            enabled: false,
            players: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub pos: usize,
    pub username: String,
    pub color: String, //Color
    pub wpm: f64,
    pub finished: bool,
}

impl Player {
    pub fn new(username: String, color: String) -> Self {
        Self {
            pos: 0,
            color,
            wpm: 0.0,
            username,
            finished: false,
        }
    }
}

pub struct App {
    pub state: State,
    pub config: Config,
    pub input_string: String,
    pub time_taken: u128,
    pub timer: Option<Instant>,
    pub connection: Connection,
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
    pub fn new() -> Self {
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
            connection: Connection::none(),
            should_exit: false,
            wordlist: Wordlist::new(Vec::new()),
            chunks: vec![],
        }
    }

    pub fn set_players(&mut self, players: Vec<Player>) {
        self.connection.players = players;
    }
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
    
    pub async fn close_connection(&mut self) {
        self.connection.tcp = None;
        self.connection.enabled = false;
    }

    pub async fn connect(&mut self, event_tx: Sender<IOEvent>) -> Result<(), std::io::Error> {
        let stream = TcpStream::connect(self.input_string.clone()).await;

        if let Err(e) = stream {
            self.current_error = e.to_string();
            return Err(e);
        }

        let stream = stream.unwrap();
        let (mut read, mut write) = stream.into_split();
        self.state = State::Waiting;

        let username = self.config.multiplayer.username.clone();
        let color = self.config.multiplayer.color.clone();
        let join_message = Message::Join(UserData::new(username, color, 0.0));

        write.write(join_message.to_string().as_bytes()).await.unwrap();

        tokio::spawn(async move {
            loop {
                let mut buf = vec![0u8; 1024];
    
                if read.read(&mut buf).await.is_err() {
                    break;
                }
    
                buf.retain(|byte| byte != &u8::MIN);
    
                if !buf.is_empty() {
                    let data = String::from_utf8(buf).unwrap();

                    if let Err(e) = event_tx.send(IOEvent::ServerMessage(data)) {
                        panic!("{}", e.to_string());
                    }
                }
            }
        });

        self.connection = Connection::new(write);
        self.connection.enabled = true;
        Ok(())
    }

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
