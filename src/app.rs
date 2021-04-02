pub struct App {
    state: State
}

pub enum State {
    StartScreen,
    EndScreen,
    TypingGame,
    TermGame
}

impl App {
    pub fn default() -> Self {
        Self {
            state: State::StartScreen,
        }
    }
}
