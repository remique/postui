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
            list_component: ListComponent::new(String::from("./src/config.json")),
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> std::io::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(f.size());

        self.list_component.draw(f, chunks[0])?;

        Ok(())
    }
}
