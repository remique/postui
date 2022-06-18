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
    list_tree: StatefulList,
    pub focused: bool,
}

struct StatefulList {
    state: ListState,
    tree: Rc<RefCell<FolderTree>>,
    items: Vec<Item>,
}

impl StatefulList {
    fn from_path<P: AsRef<std::path::Path>>(path: P) -> StatefulList {
        let foo = fs::read_to_string(path).unwrap();

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

    fn fold_folder(&mut self) {
        match self.state.selected() {
            Some(i) => {
                let current_item = &self.items.get(i).unwrap().obj_ref;

                self.tree.borrow_mut().fold_folder(current_item.as_str());
            }
            None => {}
        };
    }

    fn unfold_folder(&mut self) {
        match self.state.selected() {
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
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            file_path: format!("costam"),
            list_tree: StatefulList::from_path(path),
            focused: false,
        }
    }

    pub fn event(&mut self, ev: KeyEvent) -> bool {
        match ev.code {
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

        false
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
                .map(|item| match item {
                    // This is a bit ugly, but okay
                    &"POST " => {
                        Span::styled(String::from(*item), Style::default().fg(Color::Green))
                    }
                    &"GET " => {
                        Span::styled(String::from(*item), Style::default().fg(Color::LightYellow))
                    }
                    &"PUT " => Span::styled(String::from(*item), Style::default().fg(Color::Blue)),
                    &"DELETE " => {
                        Span::styled(String::from(*item), Style::default().fg(Color::Red))
                    }
                    _ => Span::styled(String::from(*item), Style::default()),
                })
                .collect::<Vec<Span>>();

            items.push(ListItem::new(vec![Spans::from(inside)]).style(style))
        }

        let border_type = match self.focused {
            true => BorderType::Thick,
            false => BorderType::Plain,
        };

        let the_list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(border_type),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            );

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
