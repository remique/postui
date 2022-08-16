use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders},
    Frame,
};
use tui_logger::TuiLoggerWidget;

pub struct DebugTab {}

impl DebugTab {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(r);

        let temp_block = TuiLoggerWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Gray))
            .style_info(Style::default().fg(Color::Blue))
            .block(
                Block::default()
                    .title("Logs")
                    .border_style(Style::default().fg(Color::White).bg(Color::Black))
                    .borders(Borders::ALL),
            )
            .style(Style::default().fg(Color::White).bg(Color::Black));

        f.render_widget(temp_block, chunks[0]);
    }
}
