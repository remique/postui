use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Paragraph},
    Frame,
};

pub struct CommandComponent {
    tab: usize,
}

#[derive(Clone)]
enum CommandType {
    // Command containing a value
    Command(String),

    // Space commands evenly
    Break,
}

impl CommandComponent {
    pub fn new() -> Self {
        Self { tab: 0 }
    }

    // TODO: Do not hardcode commands, but get them from components
    fn get_cmds(&self) -> Vec<CommandType> {
        match self.tab {
            0 => vec![
                CommandType::Command(String::from("Test command [a]")),
                CommandType::Break,
                CommandType::Command(String::from("Second [b]")),
                CommandType::Break,
                CommandType::Command(String::from("Reset item [c]")),
            ],
            1 => vec![
                CommandType::Command(String::from("Another page [d]")),
                CommandType::Break,
                CommandType::Command(String::from("Whatever [b]")),
                CommandType::Break,
                CommandType::Command(String::from("Push [p]")),
                CommandType::Break,
                CommandType::Command(String::from("Move [m]")),
            ],
            _ => vec![],
        }
    }

    pub fn update_cmd(&mut self, current_tab: usize) {
        self.tab = current_tab;
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let cloned = self.get_cmds().clone();

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
