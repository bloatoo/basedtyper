use tui::layout::{Constraint, Direction, Layout, Rect};
use crossterm::{execute, terminal::{LeaveAlternateScreen, disable_raw_mode}};

use super::{config::Config, parser::Word};
use std::{path::Path, time::Instant, io};

pub struct App {
    pub state: State,
    pub config: Config,
    pub input_string: String,
    pub time_taken: u128,
    pub timer: Option<Instant>,
    pub current_index: usize,
    pub current_error: String,
    pub words: Vec<Word>,
    pub should_exit: bool,
    pub word_string: String,
    pub wordlist: (bool, String),
    pub host: (bool, String),
    pub chunks: Vec<Rect>
}

pub enum State {
    MainMenu,
    EndScreen,
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
            words: Vec::new(),
            should_exit: false,
            word_string: String::new(),
            wordlist: (false, String::new()),
            host: (false, String::new()),
            chunks,
        }
    }

    pub fn restart(&mut self, state: State) {
        self.input_string = String::new();
        self.current_index = 1;
        self.time_taken = 0;
        self.current_error = String::new();
        self.wordlist = (false, String::new());
        self.host = (false, String::new());

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
        execute!(stdout, LeaveAlternateScreen)?;
        Ok(())
    }
    pub fn locate_wordlist(&self) -> String {
        let wordlist_name = if self.wordlist.1.ends_with(".basedtyper") {
            self.wordlist.1.to_string()
        } else {
            String::from(self.wordlist.1.clone()) + ".basedtyper"
        };

        let path_str = &(self.config.wordlist_directory.clone() + "/" + &wordlist_name);
        let path = Path::new(path_str);

        let path = if path.is_file() {
            path.to_str().unwrap().to_string()
        } else {
            wordlist_name.to_string()
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