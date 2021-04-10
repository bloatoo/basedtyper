//possible future implementation
use tui::{text::{Span, Spans}, widgets::Paragraph};
use crate::parser::Word;
use super::utils::Render;

pub struct Wordlist {
    pub words: Vec<Word>,
    pub current_index: usize,
}

impl Render for Wordlist {
    fn render(&self) -> Vec<Span> {
        Vec::new()
    }
}

impl Wordlist {
    pub fn new(words: Vec<Word>) -> Self {
        Self {
            words,
            current_index: 1,
        }
    }

    pub fn render() -> Paragraph<'static> {
        Paragraph::new(Spans::default())
    }
}
