use serde_json::{Map, Value};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use crate::components::CommandType;

pub struct MainPaneComponent<'a> {
    pub focused: bool,
    pub current_endpoint: Map<String, Value>,
    pub body_textbox: TextArea<'a>,
}

impl MainPaneComponent<'_> {
    pub fn new() -> Self {
        Self {
            focused: true,
            current_endpoint: Map::new(),
            body_textbox: TextArea::default(),
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(r);

        let lower_inner = chunks[1].inner(&Margin {
            vertical: 1,
            horizontal: 1,
        });

        let inside = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(lower_inner);

        let border_type = match self.focused {
            true => BorderType::Thick,
            false => BorderType::Plain,
        };

        let mut curr = String::new();
        let mut body: Vec<&str> = vec![""];

        if !self.current_endpoint.is_empty() {
            curr = format!(
                "{} | {}",
                self.current_endpoint["method"], self.current_endpoint["url"]
            );
            body = self.current_endpoint["json_body"]
                .as_str()
                .unwrap()
                .lines()
                .collect();
        }

        let tmp_vect = vec![curr.as_str()];
        let spans_inside = tmp_vect
            .iter()
            .map(|item| Span::raw(item.replace("\"", "")))
            .collect::<Vec<Span>>();

        let text = Spans::from(spans_inside);

        let temp_block = Paragraph::new(text).block(Block::default().borders(Borders::ALL));

        let temp_block2 = Block::default()
            .borders(Borders::ALL)
            .border_type(border_type);

        self.body_textbox = TextArea::from(body);

        self.body_textbox
            .set_cursor_style(Style::default().bg(Color::Black));

        let body_widget = self.body_textbox.widget();

        f.render_widget(temp_block, chunks[0]);
        f.render_widget(temp_block2, chunks[1]);

        f.render_widget(body_widget, inside[0]);
    }

    pub fn generate_cmds(&self) -> Vec<CommandType> {
        vec![
            CommandType::Command(String::from("main [o]")),
            CommandType::Break,
            CommandType::Command(String::from("pane [x]")),
        ]
    }
}
