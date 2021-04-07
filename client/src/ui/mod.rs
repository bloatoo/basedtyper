use tui::{backend::Backend, layout::{Alignment, Constraint, Direction, Layout}, terminal::Frame, text::Spans, widgets::Paragraph};

pub fn centered_spans<'a>(size: &u16) -> Vec<Spans> {
    let mut v = Vec::new();

    for _ in 0..*size {
        v.push(Spans::default());    
    }

    v
}

pub fn draw<T: Backend>(f: &mut Frame<T>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Percentage(100)])
        .split(f.size());

    let a = Paragraph::new(centered_spans(&chunks[0].height))
        .alignment(Alignment::Center);
}
