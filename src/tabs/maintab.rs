use crossterm::event::KeyEvent;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

use crate::components::ListComponent;

pub struct MainTab {
    list_component: ListComponent,
}

impl MainTab {
    pub fn new() -> Self {
        Self {
            list_component: ListComponent::new("./config.json"),
        }
    }

    pub fn event(&mut self, ev: KeyEvent) {
        self.list_component.event(ev);
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(r);

        // TODO: Make a proper main window
        let temp_block = Block::default().borders(Borders::ALL);
        f.render_widget(temp_block, chunks[1]);

        self.list_component.draw(f, chunks[0]);
    }
}
