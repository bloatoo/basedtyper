use std::time::Instant;

pub struct App {
    pub state: State,
    pub input_string: String,
    pub time_taken: u128,
    pub timer: Option<Instant>,
    pub current_index: usize,
}

pub enum State {
    MainMenu,
    EndScreen,
    TypingGame,
    TermGame
}

impl App {
    pub fn default() -> Self {
        Self {
            state: State::MainMenu,
            input_string: String::new(),
            timer: None,
            time_taken: 0,
            current_index: 1,
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
}
