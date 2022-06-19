use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::components::{CommandType, ListComponent, MainPaneComponent};

pub struct MainTab {
    list_component: ListComponent,
    main_pane: MainPaneComponent,
    focus: Focus,
    pub current_cmds: Vec<CommandType>,
}

#[derive(PartialEq)]
enum Focus {
    FolderTreeWindow,
    MainPane, // This will be changed later on
}

impl MainTab {
    pub fn new() -> Self {
        let list_component = ListComponent::new("./config.json");
        let current_cmds = list_component.generate_cmds();

        Self {
            list_component,
            current_cmds,
            main_pane: MainPaneComponent::new(),
            focus: Focus::MainPane,
        }
    }

    pub fn event(&mut self, ev: KeyEvent) {
        // TODO: This shit needs refactor
        match self.focus {
            Focus::FolderTreeWindow => {
                let can_unfold = self.list_component.list_tree.can_unfold_folder();

                if ev.code == KeyCode::Right && can_unfold == false {
                    self.focus = Focus::MainPane;
                    self.list_component.focused = false;
                    self.main_pane.focused = true;
                    self.current_cmds = self.main_pane.generate_cmds();
                } else {
                    self.list_component.event(ev);
                }
            }
            Focus::MainPane => {
                if ev.code == KeyCode::Left {
                    self.focus = Focus::FolderTreeWindow;
                    self.list_component.focused = true;
                    self.main_pane.focused = false;
                    self.current_cmds = self.list_component.generate_cmds();
                } else {
                    // self.main_pane.event(ev);
                }
            }
        };
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(r);

        self.list_component.draw(f, chunks[0]);
        self.main_pane.draw(f, chunks[1]);
    }
}
