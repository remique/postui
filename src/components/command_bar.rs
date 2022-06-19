use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Paragraph},
    Frame,
};

#[derive(Clone)]
pub enum CommandType {
    // Command containing a value
    Command(String),

    // Space commands evenly
    Break,
}

pub struct CommandComponent {
    tab: usize,
    list: Vec<CommandType>,
}

impl CommandComponent {
    pub fn new(list: Vec<CommandType>) -> Self {
        Self { tab: 0, list }
    }

    pub fn cmds_from(&mut self, input: Vec<CommandType>) {
        self.list = input;
    }

    // TODO: Do not hardcode commands, but get them from components
    fn get_cmds(&self) -> Vec<CommandType> {
        self.list.clone()
    }

    pub fn update_cmd(&mut self, current_tab: usize) {
        self.tab = current_tab;
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let cloned = self.get_cmds();

        let spans_inside = cloned
            .iter()
            .map(|item| match item {
                CommandType::Command(val) => {
                    Span::styled(val, Style::default().bg(Color::Blue).fg(Color::Black))
                }
                CommandType::Break => Span::raw(" "),
            })
            .collect::<Vec<Span>>();

        let text = Spans::from(spans_inside);

        let cmds = Paragraph::new(text.clone())
            .block(Block::default())
            .alignment(Alignment::Left);

        f.render_widget(cmds, r);
    }
}
