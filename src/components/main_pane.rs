use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::components::CommandType;

pub struct MainPaneComponent {
    pub focused: bool,
    pub current_endpoint: String,
}

impl MainPaneComponent {
    pub fn new() -> Self {
        Self {
            focused: true,
            current_endpoint: String::from("asdf"),
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(r);

        let border_type = match self.focused {
            true => BorderType::Thick,
            false => BorderType::Plain,
        };

        let tmp_vect = vec![self.current_endpoint.as_str()];
        let spans_inside = tmp_vect
            .iter()
            .map(|item| Span::raw(item.replace("\"", "")))
            .collect::<Vec<Span>>();

        let text = Spans::from(spans_inside);

        let temp_block = Paragraph::new(text).block(Block::default().borders(Borders::ALL));

        let temp_block2 = Block::default()
            .borders(Borders::ALL)
            .border_type(border_type);

        f.render_widget(temp_block, chunks[0]);
        f.render_widget(temp_block2, chunks[1]);
    }

    pub fn generate_cmds(&self) -> Vec<CommandType> {
        vec![
            CommandType::Command(String::from("main [o]")),
            CommandType::Break,
            CommandType::Command(String::from("pane [x]")),
        ]
    }
}
