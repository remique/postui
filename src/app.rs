use tui::backend::Backend;
use tui::layout::{Constraint, Corner, Direction, Layout};
use tui::Frame;

use crate::components::ListComponent;

pub struct App {
    list_component: ListComponent,
}

impl App {
    pub fn new() -> Self {
        Self {
            list_component: ListComponent::new(String::from("./config.json")),
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> std::io::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2), // Tabs
                Constraint::Min(2),    // Main
                Constraint::Length(2), // Cmdbar
            ])
            .split(f.size());

        self.list_component.draw(f, chunks[1])?;

        Ok(())
    }
}
