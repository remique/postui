use crossterm::event::{KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::components::{CommandType, FolderPopup, ListComponent, MainPaneComponent};

pub struct MainTab<'a> {
    list_component: ListComponent,
    main_pane: MainPaneComponent,
    folder_popup: FolderPopup<'a>,
    focus: Focus,
    pub current_cmds: Vec<CommandType>,
}

#[derive(PartialEq)]
enum Focus {
    FolderTreeWindow,
    MainPane, // This will be changed later on
    FolderPopup,
}

impl MainTab<'_> {
    pub fn new() -> Self {
        let list_component = ListComponent::new("./config.json");
        let current_cmds = list_component.generate_cmds();

        Self {
            list_component,
            current_cmds,
            folder_popup: FolderPopup::new(),
            main_pane: MainPaneComponent::new(),
            focus: Focus::MainPane,
        }
    }

    pub fn event(&mut self, ev: KeyEvent) {
        // TODO: This shit needs refactor
        match self.focus {
            Focus::FolderTreeWindow => {
                let can_unfold = self.list_component.list_tree.can_unfold_folder();

                if ev.code == KeyCode::Right && !can_unfold {
                    self.switch_focus(Focus::MainPane);
                    self.current_cmds = self.main_pane.generate_cmds();
                } else {
                    self.list_component.event(ev);

                    if let Some(i) = self.list_component.list_tree.get_current_endpoint() {
                        self.main_pane.current_endpoint = i;
                    }
                }
                if ev.code == KeyCode::Char('a') {
                    self.folder_popup.is_open = !self.folder_popup.is_open;
                    self.switch_focus(Focus::FolderPopup);
                }
            }
            Focus::MainPane => {
                if ev.code == KeyCode::Left {
                    self.switch_focus(Focus::FolderTreeWindow);
                    self.current_cmds = self.list_component.generate_cmds();
                } else {
                    // self.main_pane.event(ev);
                }
            }
            Focus::FolderPopup => {
                if ev.code == KeyCode::Esc {
                    self.folder_popup.close();
                    self.switch_focus(Focus::FolderTreeWindow);
                }

                self.folder_popup.event(ev);

                if self.folder_popup.is_saved() == true {
                    self.folder_popup.close();
                    self.switch_focus(Focus::FolderTreeWindow);
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

        let centered = self.folder_popup.centered_rect(60, 60, f.size());
        self.folder_popup.draw(f, centered);
    }

    fn switch_focus(&mut self, f: Focus) {
        match f {
            Focus::FolderTreeWindow => {
                self.list_component.focused = true;
                self.main_pane.focused = false;
            }
            Focus::MainPane => {
                self.list_component.focused = false;
                self.main_pane.focused = true;
            }
            Focus::FolderPopup => {
                self.list_component.focused = false;
                self.main_pane.focused = false;
            }
        }

        self.focus = f;
    }
}
