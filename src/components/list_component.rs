use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::widgets::{Block, BorderType, Borders, List, ListItem};
use tui::Frame;

use crate::foldertree::FolderTree;

use std::fs;

pub struct ListComponent {
    tree: FolderTree,
    file_path: String,
}

impl ListComponent {
    pub fn new(path: String) -> Self {
        let foo = fs::read_to_string(path.as_str()).unwrap();

        let ft = FolderTree::from_str(foo.as_str());
        ft.parse_all();

        Self {
            tree: ft,
            file_path: path,
        }
    }

    fn create_list_vec(&self) -> Vec<ListItem> {
        let mut myvec: Vec<ListItem> = Vec::new();

        for item in self.tree.items.borrow().iter() {
            myvec.push(ListItem::new(format!("{}", item.rep.as_str())))
        }

        myvec
    }

    pub fn draw<B: Backend>(&self, f: &mut Frame<B>, r: Rect) -> std::io::Result<()> {
        let items = self.create_list_vec();

        let the_list = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC));

        f.render_widget(the_list, r);

        Ok(())
    }
}
