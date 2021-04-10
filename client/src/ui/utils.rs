use tui::{layout::Alignment, text::{Span, Spans}, widgets::Paragraph};

pub trait Render {
    fn render(&self) -> Vec<Span>;
}
pub fn spans(size: u16) -> Vec<Spans<'static>> {
    let mut v = Vec::new();

    for _ in 0..size {
        v.push(Spans::default());    
    }

    v
}

pub fn center(spans: Vec<Spans<'static>>) -> Paragraph<'static> {
    Paragraph::new(spans).alignment(Alignment::Center)
}
