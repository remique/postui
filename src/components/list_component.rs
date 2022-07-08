use crossterm::event::{KeyCode, KeyEvent};
use std::{cell::RefCell, fs, path::Path, rc::Rc};
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};

use crate::components::CommandType;
use crate::foldertree::{FolderTree, Item};

pub struct ListComponent {
    file_path: String,
    pub list_tree: StatefulList,
    pub focused: bool,
}

pub struct StatefulList {
    state: ListState,
    tree: Rc<RefCell<FolderTree>>,
    items: Vec<Item>,
}

impl StatefulList {
    fn from_path<P: AsRef<std::path::Path>>(path: P) -> StatefulList {
        let path_to_string = fs::read_to_string(path).unwrap();

        let ft = FolderTree::from_str(path_to_string.as_str());
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
                if i >= self.tree.borrow().items.borrow().len() - 1 {
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
                    self.tree.borrow().items.borrow().len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn get_current_endpoint(&self) -> Option<String> {
        if let Some(i) = self.state.selected() {
            let current_item = &self.items.get(i).unwrap().obj_ref;

            return self
                .tree
                .borrow()
                .get_current_from_path(current_item.as_str());
        }

        None
    }

    fn can_fold_folder(&self) -> bool {
        if let Some(i) = self.state.selected() {
            let current_item = &self.items.get(i).unwrap().obj_ref;

            return self.tree.borrow().can_fold_folder(current_item.as_str());
        }

        false
    }

    pub fn can_unfold_folder(&self) -> bool {
        if let Some(i) = self.state.selected() {
            let current_item = &self.items.get(i).unwrap().obj_ref;

            return self.tree.borrow().can_unfold_folder(current_item.as_str());
        }

        false
    }

    fn fold_folder(&mut self) {
        if let Some(i) = self.state.selected() {
            let current_item = &self.items.get(i).unwrap().obj_ref;

            self.tree.borrow_mut().fold_folder(current_item.as_str());
        }
    }

    fn unfold_folder(&mut self) {
        if let Some(i) = self.state.selected() {
            let current_item = &self.items.get(i).unwrap().obj_ref;

            self.tree.borrow_mut().unfold_folder(current_item.as_str());
        }
    }
}

impl ListComponent {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            file_path: "temp_file_path".to_string(),
            list_tree: StatefulList::from_path(path),
            focused: false,
        }
    }

    pub fn event(&mut self, ev: KeyEvent) {
        match ev.code {
            KeyCode::Down => {
                self.list_tree.next();
            }
            KeyCode::Up => {
                self.list_tree.previous();
            }
            KeyCode::Left => {
                self.list_tree.fold_folder();
            }
            KeyCode::Right => {
                self.list_tree.unfold_folder();
            }
            _ => {}
        };
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let mut items: Vec<ListItem> = Vec::new();

        for item in self.list_tree.tree.borrow().items.borrow().iter() {
            let style = match item.r#type.as_str() {
                "folder" => Style::default().add_modifier(Modifier::BOLD),
                "endpoint" => Style::default(),
                _ => Style::default(),
            };

            let split_item = item.rep.split_inclusive(" ").collect::<Vec<&str>>();

            let inside = split_item
                .iter()
                .map(|item| match *item {
                    // This is a bit ugly, but okay
                    "POST " => Span::styled(String::from(*item), Style::default().fg(Color::Green)),
                    "GET " => {
                        Span::styled(String::from(*item), Style::default().fg(Color::LightYellow))
                    }
                    "PUT " => Span::styled(String::from(*item), Style::default().fg(Color::Blue)),
                    "DELETE " => Span::styled(String::from(*item), Style::default().fg(Color::Red)),
                    _ => Span::styled(String::from(*item), Style::default()),
                })
                .collect::<Vec<Span>>();

            items.push(ListItem::new(vec![Spans::from(inside)]).style(style))
        }

        let border_type = match self.focused {
            true => BorderType::Thick,
            false => BorderType::Plain,
        };

        let highlight_style = if self.focused {
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().add_modifier(Modifier::BOLD)
        };

        let the_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(border_type),
            )
            .highlight_style(highlight_style);

        f.render_stateful_widget(the_list, r, &mut self.list_tree.state);
    }

    pub fn generate_cmds(&self) -> Vec<CommandType> {
        vec![
            CommandType::Command(String::from("hehe [x]")),
            CommandType::Break,
            CommandType::Command(String::from("lmao [y]")),
        ]
    }
}
