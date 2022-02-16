use tui::backend::Backend;
use tui::Frame;

use crate::components::ListComponent;

pub struct App {
    list_component: ListComponent,
}

impl App {
    pub fn new() -> Self {
        Self {
            list_component: ListComponent::new(),
        }
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>) -> std::io::Result<()> {
        self.list_component.draw(f)?;

        Ok(())
    }
}
