use tui::style::Color;

pub struct Word {
    word: String,
    definition: String,
    _color: Color,
}

impl Word {
    pub fn new<T: ToString>(word: T, definition: T) -> Self {
        let word = word.to_string();
        let definition = definition.to_string();

        Self {
            word,
            definition,
            _color: Color::DarkGray,
        }
    }

    pub fn get_definition(&self) -> &String {
        &self.definition
    }

    pub fn get_word(&self) -> &String {
        &self.word
    }
}
