use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::components::CommandType;

pub struct MainPaneComponent {
    pub focused: bool,
}

impl MainPaneComponent {
    pub fn new() -> Self {
        Self { focused: false }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let border_type = match self.focused {
            true => BorderType::Thick,
            false => BorderType::Plain,
        };

        let temp_block = Block::default()
            .borders(Borders::ALL)
            .border_type(border_type);

        f.render_widget(temp_block, r);
    }

    pub fn generate_cmds(&self) -> Vec<CommandType> {
        vec![
            CommandType::Command(String::from("main [o]")),
            CommandType::Break,
            CommandType::Command(String::from("pane [x]")),
        ]
    }
}
