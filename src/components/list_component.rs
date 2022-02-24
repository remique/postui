use crossterm::event::{self, Event, KeyCode};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, ListState};
use tui::Frame;

use crate::foldertree::FolderTree;

use std::fs;

pub struct ListComponent {
    tree: FolderTree,
    file_path: String,
    list_tree: StatefulList<String>,
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
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

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

impl ListComponent {
    pub fn new(path: String) -> Self {
        let foo = fs::read_to_string(path.as_str()).unwrap();

        let ft = FolderTree::from_str(foo.as_str());
        ft.parse_all();

        let mut myvec: Vec<String> = Vec::new();

        for item in ft.items.borrow().iter() {
            myvec.push(format!("{}", item.rep));
        }

        Self {
            tree: ft,
            file_path: path,
            list_tree: StatefulList::with_items(myvec),
        }
    }

    fn create_list_vec(&self) -> Vec<ListItem> {
        let mut myvec: Vec<ListItem> = Vec::new();

        for item in self.tree.items.borrow().iter() {
            myvec.push(ListItem::new(format!("{}", item.rep.as_str())))
        }

        myvec
    }

    pub fn event(&mut self, ev: Event) -> bool {
        if let Event::Key(e) = ev {
            return match e.code {
                KeyCode::Down => {
                    // println!("XD!");
                    self.list_tree.next();
                    true
                }
                KeyCode::Up => {
                    // println!(":(!");
                    self.list_tree.previous();
                    true
                }
                KeyCode::Left => {
                    self.list_tree.next();
                    true
                }
                _ => false,
            };
        }

        false
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) -> std::io::Result<()> {
        let mut items: Vec<ListItem> = Vec::new();

        for item in self.tree.items.borrow().iter() {
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
