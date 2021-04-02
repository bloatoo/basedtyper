pub struct App {
    pub state: State,
    pub input_string: String,
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
        }
    }
}
