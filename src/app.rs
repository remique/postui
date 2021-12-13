use tui::backend::Backend;
use tui::Frame;

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> std::io::Result<()> {
        Ok(())
    }
}
