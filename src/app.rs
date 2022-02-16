use tui::backend::Backend;
use tui::Frame;

// To delete potem
use tui::widgets::{Block, BorderType, Borders};

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> std::io::Result<()> {
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Main block with round corners")
            .border_type(BorderType::Rounded);

        f.render_widget(block, f.size());

        Ok(())
    }
}
