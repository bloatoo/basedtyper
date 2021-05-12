#[derive(Clone)]
pub struct Word {
    word: String,
    definition: String,
}

impl Word {
    pub fn new<T: ToString>(word: T, definition: T) -> Self {
        let word = word.to_string();
        let definition = definition.to_string();

        Self {
            word,
            definition,
        }
    }

    pub fn get_definition(&self) -> &String {
        &self.definition
    }

    pub fn get_word(&self) -> &String {
        &self.word
    }
}
