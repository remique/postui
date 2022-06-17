use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::components::ListComponent;

pub struct MainTab {
    list_component: ListComponent,
    focus: Focus,
}

#[derive(PartialEq)]
enum Focus {
    FolderTreeWindow,
    MainWindow, // This will be changed later on
}

impl MainTab {
    pub fn new() -> Self {
        Self {
            list_component: ListComponent::new("./config.json"),
            focus: Focus::MainWindow,
        }
    }

    pub fn event(&mut self, ev: KeyEvent) {
        match ev.code {
            KeyCode::Left => {
                self.focus = Focus::FolderTreeWindow;
                self.list_component.focused = true;
            }
            KeyCode::Right => {
                self.focus = Focus::MainWindow;
                self.list_component.focused = false;
            }
            _ => {}
        };

        // if self.focus == Focus::FolderTreeWindow {
        //     self.list_component.event(ev);
        // }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(r);

        let block_border_type = match self.focus {
            Focus::MainWindow => BorderType::Thick,
            Focus::FolderTreeWindow => BorderType::Plain,
        };

        // TODO: Make a proper main window
        let temp_block = Block::default()
            .borders(Borders::ALL)
            .border_type(block_border_type);

        f.render_widget(temp_block, chunks[1]);

        self.list_component.draw(f, chunks[0]);
    }
}
