use crossterm::event::{self, Event, KeyCode};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState};
use tui::Frame;

use crate::foldertree::{FolderTree, Item};

use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

pub struct ListComponent {
    file_path: String,
    list_tree: StatefulList,
}

struct StatefulList {
    state: ListState,
    tree: Rc<RefCell<FolderTree>>,
    items: Vec<Item>,
}

impl StatefulList {
    fn from_path(path: String) -> StatefulList {
        let foo = fs::read_to_string(path.as_str()).unwrap();

        let ft = FolderTree::from_str(foo.as_str());
        ft.parse_all();

        let mut new_state = ListState::default();
        new_state.select(Some(0));

        let items = ft.items.borrow().clone();

        StatefulList {
            state: new_state,
            tree: Rc::new(RefCell::new(ft)),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn fold_folder(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                let current_item = &self.items.get(i).unwrap().obj_ref;

                self.tree.borrow_mut().fold_folder(current_item.as_str());
            }
            None => {}
        };
    }

    fn unfold_folder(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                let current_item = &self.items.get(i).unwrap().obj_ref;

                self.tree.borrow_mut().unfold_folder(current_item.as_str());
            }
            None => {}
        };
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl ListComponent {
    pub fn new(path: String) -> Self {
        let foo = fs::read_to_string(path.as_str()).unwrap();

        Self {
            file_path: format!("costam"),
            list_tree: StatefulList::from_path(path),
        }
    }

    pub fn event(&mut self, ev: Event) -> bool {
        if let Event::Key(e) = ev {
            return match e.code {
                KeyCode::Down => {
                    self.list_tree.next();
                    true
                }
                KeyCode::Up => {
                    self.list_tree.previous();
                    true
                }
                KeyCode::Left => {
                    self.list_tree.fold_folder();
                    true
                }
                KeyCode::Right => {
                    self.list_tree.unfold_folder();
                    true
                }
                _ => false,
            };
        }

        false
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) -> std::io::Result<()> {
        let mut items: Vec<ListItem> = Vec::new();

        for item in self.list_tree.tree.borrow().items.borrow().iter() {
            items.push(ListItem::new(format!("{}", item.rep.as_str())))
        }

        let the_list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

        f.render_stateful_widget(the_list, r, &mut self.list_tree.state);

        Ok(())
    }
}
