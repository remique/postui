use crossterm::event::{KeyCode, KeyEvent};
use std::path::Path;
use tui::{
    backend::Backend,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};

use crate::components::CommandType;
use crate::foldertree::FolderTree;

pub struct ListComponent {
    list_tree: StatefulList,
    focused: bool,
}

pub struct StatefulList {
    state: ListState,
    tree: FolderTree,
}

impl StatefulList {
    fn from_path<P: AsRef<Path>>(path: P) -> StatefulList {
        let mut new_state = ListState::default();
        let tree = FolderTree::new(path).unwrap();

        new_state.select(Some(0));
        tree.parse_all();

        StatefulList {
            state: new_state,
            tree,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.tree.items.borrow().len() - 1 {
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
                    self.tree.items.borrow().len() - 1
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
            let items = self.tree.items.borrow().clone();
            let current = items.get(i).unwrap().obj_ref.as_str();

            return self.tree.get_current_from_path(current);
        }

        None
    }

    fn can_fold_folder(&self) -> bool {
        if let Some(i) = self.state.selected() {
            let items = self.tree.items.borrow().clone();
            let current = items.get(i).unwrap().obj_ref.as_str();

            return self.tree.can_fold_folder(current);
        }

        false
    }

    pub fn can_unfold_folder(&self) -> bool {
        if let Some(i) = self.state.selected() {
            let items = self.tree.items.borrow().clone();
            let current = items.get(i).unwrap().obj_ref.as_str();

            return self.tree.can_unfold_folder(current);
        }

        false
    }

    fn fold_folder(&mut self) {
        if let Some(i) = self.state.selected() {
            let items = self.tree.items.borrow().clone();
            let current = items.get(i).unwrap().obj_ref.as_str();

            self.tree.fold_folder(current);
        }
    }

    fn unfold_folder(&mut self) {
        if let Some(i) = self.state.selected() {
            let items = self.tree.items.borrow().clone();
            let current = items.get(i).unwrap().obj_ref.as_str();

            self.tree.unfold_folder(current);
        }
    }

    pub fn insert_endpoint(&mut self) {
        if let Some(i) = self.state.selected() {
            let items = self.tree.items.borrow().clone();
            let current = items.get(i).unwrap().obj_ref.as_str();

            self.tree.insert_endpoint(current, "Hehe");
        }
    }
}

impl ListComponent {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
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

        for item in self.list_tree.tree.items.borrow().iter() {
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

    pub fn tree(&mut self) -> &mut StatefulList {
        &mut self.list_tree
    }

    pub fn focused(&self) -> bool {
        self.focused
    }

    pub fn set_focus(&mut self, val: bool) {
        self.focused = val;
    }
}
