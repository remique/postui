use crossterm::event::{Event, KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders, Clear},
    Frame,
};
use tui_textarea::TextArea;

#[derive(PartialEq)]
enum CurrentContextFocus {
    EndpointName,
    EndpointURL,
}

pub struct FolderPopup<'a> {
    pub is_open: bool,
    name_textbox: TextArea<'a>,
    url_textbox: TextArea<'a>,
    current_context_focus: CurrentContextFocus,
}

impl FolderPopup<'_> {
    pub fn new() -> Self {
        Self {
            is_open: false,
            name_textbox: TextArea::default(),
            url_textbox: TextArea::default(),
            current_context_focus: CurrentContextFocus::EndpointName,
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        if self.is_open {
            let block = Block::default().borders(Borders::ALL).title("Add endpoint");
            let block_inner = block.inner(r);

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(block_inner);

            self.name_textbox.set_cursor_line_style(Style::default());
            self.url_textbox.set_cursor_line_style(Style::default());

            let name_border_type =
                if self.current_context_focus == CurrentContextFocus::EndpointName {
                    BorderType::Thick
                } else {
                    BorderType::Plain
                };

            let url_border_type = if self.current_context_focus == CurrentContextFocus::EndpointURL
            {
                BorderType::Thick
            } else {
                BorderType::Plain
            };

            self.name_textbox.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(name_border_type)
                    .title("Enter endpoint name"),
            );

            self.url_textbox.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(url_border_type)
                    .title("Enter endpoint URL"),
            );

            let name_widget = self.name_textbox.widget();
            let url_widget = self.url_textbox.widget();

            let name_layout = Layout::default()
                .constraints([Constraint::Length(3), Constraint::Min(1)].as_slice())
                .split(block_inner);

            f.render_widget(Clear, r);

            f.render_widget(block, r);
            f.render_widget(name_widget, name_layout[0]);
            f.render_widget(url_widget, layout[1]);
        }
    }

    pub fn event(&mut self, ev: KeyEvent) {
        match self.current_context_focus {
            CurrentContextFocus::EndpointName => {
                self.name_textbox.input(ev);

                if ev.code == KeyCode::Down {
                    self.current_context_focus = CurrentContextFocus::EndpointURL;
                }
            }
            CurrentContextFocus::EndpointURL => {
                self.url_textbox.input(ev);

                if ev.code == KeyCode::Up {
                    self.current_context_focus = CurrentContextFocus::EndpointName;
                }
            }
        }
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
