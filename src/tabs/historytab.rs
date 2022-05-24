use tui::{
    backend::Backend,
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

pub struct HistoryTab {}

impl HistoryTab {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(r);

        let temp_block = Block::default().borders(Borders::ALL);
        f.render_widget(temp_block, chunks[0]);
    }
}
