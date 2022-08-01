use crossterm::event::{Event, KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear},
    Frame,
};
use tui_textarea::{Input, TextArea};

pub struct FolderPopup<'a> {
    pub is_open: bool,
    textarea: TextArea<'a>,
}

impl FolderPopup<'_> {
    pub fn new() -> Self {
        Self {
            is_open: false,
            textarea: TextArea::default(),
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        if self.is_open {
            self.textarea.set_cursor_line_style(Style::default());

            let widg = self.textarea.widget();

            let block1 = Block::default().title("raz").borders(Borders::ALL);
            let block2 = Block::default().title("dwa").borders(Borders::ALL);

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(r);

            f.render_widget(Clear, r);
            f.render_widget(block1, layout[0]);
            f.render_widget(widg, layout[1]);

            // f.render_widget(widg, r);
        }
    }

    pub fn event(&mut self, ev: KeyEvent) {
        self.textarea.input(ev);
    }

    pub fn centered_rect(&self, percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_y) / 2),
                    Constraint::Percentage(percent_y),
                    Constraint::Percentage((100 - percent_y) / 2),
                ]
                .as_ref(),
            )
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage((100 - percent_x) / 2),
                    Constraint::Percentage(percent_x),
                    Constraint::Percentage((100 - percent_x) / 2),
                ]
                .as_ref(),
            )
            .split(popup_layout[1])[1]
    }
}
