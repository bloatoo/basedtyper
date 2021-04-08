use tui::text::Span;

pub trait Render {
    fn render(&self) -> Vec<Span>;
}
