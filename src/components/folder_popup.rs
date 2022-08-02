use crossterm::event::{Event, KeyCode, KeyEvent};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Clear},
    Frame,
};
use tui_textarea::TextArea;

#[derive(PartialEq)]
enum Focus {
    Name,
    URL,
    OkButton,
}

pub struct FolderPopup<'a> {
    pub is_open: bool,
    name_textbox: TextArea<'a>,
    url_textbox: TextArea<'a>,
    focus: Focus,
    pub save_endpoint: bool,
}

impl FolderPopup<'_> {
    pub fn new() -> Self {
        Self {
            is_open: false,
            name_textbox: TextArea::default(),
            url_textbox: TextArea::default(),
            focus: Focus::Name,
            save_endpoint: false,
        }
    }

    pub fn save(&mut self) {
        self.save_endpoint = true;
    }

    pub fn is_saved(&self) -> bool {
        self.save_endpoint
    }

    pub fn close(&mut self) {
        self.save_endpoint = false;
        self.is_open = false;
        self.name_textbox = TextArea::default();
        self.url_textbox = TextArea::default();
        self.focus = Focus::Name;
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        if self.is_open {
            let block = Block::default().borders(Borders::ALL).title("Add endpoint");
            let block_inner = block.inner(r);

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [Constraint::Max(10), Constraint::Max(10), Constraint::Max(2)].as_ref(),
                )
                .split(block_inner);

            self.name_textbox.set_cursor_line_style(Style::default());
            self.url_textbox.set_cursor_line_style(Style::default());

            let name_border_type = if self.focus == Focus::Name {
                BorderType::Thick
            } else {
                BorderType::Plain
            };

            let url_border_type = if self.focus == Focus::URL {
                BorderType::Thick
            } else {
                BorderType::Plain
            };

            let ok_button_fill = if self.focus == Focus::OkButton {
                Color::Gray
            } else {
                Color::Black
            };

            let ok_button = Block::default()
                .borders(Borders::ALL)
                .title("OK")
                .title_alignment(Alignment::Center)
                .style(Style::default().bg(ok_button_fill));

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
                .split(layout[0]);

            let url_layout = Layout::default()
                .constraints([Constraint::Length(3), Constraint::Min(1)].as_slice())
                .split(layout[1]);

            // We have to clear out the background first
            f.render_widget(Clear, r);

            f.render_widget(block, r);
            f.render_widget(name_widget, name_layout[0]);
            f.render_widget(url_widget, url_layout[0]);
            f.render_widget(ok_button, layout[2]);
        }
    }

    pub fn event(&mut self, ev: KeyEvent) {
        match self.focus {
            Focus::Name => {
                if ev.code != KeyCode::Up && ev.code != KeyCode::Enter {
                    self.name_textbox.input(ev);
                }

                if ev.code == KeyCode::Down {
                    self.focus = Focus::URL;
                }
            }
            Focus::URL => {
                self.url_textbox.input(ev);

                if ev.code == KeyCode::Up {
                    self.focus = Focus::Name;
                }

                if ev.code == KeyCode::Down {
                    self.focus = Focus::OkButton;
                }
            }
            Focus::OkButton => {
                if ev.code == KeyCode::Up {
                    self.focus = Focus::URL;
                }
                if ev.code == KeyCode::Enter {
                    // apply
                    self.save_endpoint = true;
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
