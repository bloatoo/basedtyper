use super::super::parser::Word;

pub struct Wordlist {
    words: Vec<Word>,
}

impl From<String> for Wordlist {
    fn from(data: String) -> Self {
        let words = data
            .split(' ')
            .map(|word| Word::new(word, ""))
            .collect();

        Self {
            words,
        }
    }
}

impl From<Vec<Word>> for Wordlist {
    fn from(words: Vec<Word>) -> Self {
        Self {
            words
        }
    }
}

impl ToString for Wordlist {
    fn to_string(&self) -> String {
        self.words
            .iter()
            .map(|elem| elem.get_word().clone())
            .collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string()
    }
}

impl Wordlist {
    pub fn new(words: Vec<Word>) -> Self {
        Self {
            words,
        }
    }
    
    pub fn defs(&self) -> Vec<String> {
        self.words
            .iter()
            .map(|elem| elem.get_definition().clone())
            .collect()
    }
    
    pub fn words(&self) -> Vec<String> {
        self.words
            .iter()
            .map(|elem| elem.get_word().clone())
            .collect()
    }
    pub fn resize(&mut self, width: u16) {
        let before = self.to_string();
        if before.len() > width as usize {
            self.words = Self::from(before.split_at(width as usize).0.to_string()).words;
        }
    }
}
