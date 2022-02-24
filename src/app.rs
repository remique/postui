use crossterm::event::{self, Event, KeyCode};
use tui::backend::Backend;
use tui::layout::{Constraint, Corner, Direction, Layout};
use tui::Frame;

use crate::components::ListComponent;

pub struct App {
    list_component: ListComponent,
    do_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            list_component: ListComponent::new(String::from("./config.json")),
            do_quit: false,
        }
    }

    pub fn event(&mut self, ev: Event) {
        if self.list_component.event(ev) {
            return;
        }

        if ev == Event::Key(KeyCode::Char('q').into()) {
            self.do_quit = true;
            return;
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) -> std::io::Result<()> {
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

    pub fn is_quit(&self) -> bool {
        self.do_quit
    }
}
