use crossterm::event::{self, Event};
use tui::{
    backend::Backend,
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::components::ListComponent;

pub struct MainTab {
    list_component: ListComponent,
}

impl MainTab {
    pub fn new() -> Self {
        Self {
            list_component: ListComponent::new(String::from("./config.json")),
        }
    }

    pub fn event(&mut self, ev: Event) {
        if self.list_component.event(ev) {
            return;
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(r);

        // TODO: Make a proper main window
        let temp_block = Block::default().borders(Borders::ALL);
        f.render_widget(temp_block, chunks[1]);

        self.list_component.draw(f, chunks[0]).unwrap();
    }
}
