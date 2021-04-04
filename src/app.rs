use super::{config::Config, parser::Word};
use std::{path::Path, time::Instant};

pub struct App {
    pub state: State,
    pub config: Config,
    pub input_string: String,
    pub time_taken: u128,
    pub timer: Option<Instant>,
    pub current_index: usize,
    pub current_error: String,
    pub words: Vec<Word>,
}

pub enum State {
    MainMenu,
    EndScreen,
    TypingGame,
    TermGame
}

impl App {
    pub fn default() -> Self {
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
        }
    }

    pub fn restart(&mut self, state: State) {
        self.state = state;
        self.timer = None;
        self.input_string = String::new();
        self.current_index = 1;
        self.time_taken = 0;
    }

    pub fn start_timer(&mut self) {
        self.timer = Some(Instant::now());
    }

    pub fn locate_wordlist(&self, wordlist_name: &str) -> String {
        let wordlist_name = if wordlist_name.ends_with(".basedtyper") {
            wordlist_name.to_string()
        } else {
            String::from(wordlist_name) + ".basedtyper"
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
