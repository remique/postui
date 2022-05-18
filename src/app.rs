use crossterm::event::{self, Event, KeyCode};

use tui::{
    backend::Backend,
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::components::ListComponent;
use crate::tabs::MainTab;

pub struct App {
    main_tab: MainTab,
    do_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            main_tab: MainTab::new(),
            do_quit: false,
        }
    }

    pub fn event(&mut self, ev: Event) {
        if ev == Event::Key(KeyCode::Char('q').into()) {
            self.do_quit = true;
            return;
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) -> std::io::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Tabs
                Constraint::Min(2),    // Main
                Constraint::Length(2), // Cmdbar
            ])
            .split(f.size());

        self.draw_tabs(f, chunks[0]);

        self.main_tab.draw(f, chunks[1]);

        Ok(())
    }

    pub fn is_quit(&self) -> bool {
        self.do_quit
    }

    pub fn draw_tabs<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        let titles = vec![
            String::from("Main"),
            String::from("History"),
            String::from("About"),
        ];

        let titles = titles
            .iter()
            .map(|item| Spans::from(vec![Span::styled(item, Style::default().fg(Color::Yellow))]))
            .collect();

        let tabs = Tabs::new(titles).block(Block::default().borders(Borders::ALL));

        f.render_widget(tabs, r);
    }
}
