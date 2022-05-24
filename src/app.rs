use crossterm::event::{self, Event, KeyCode, KeyEvent};

use tui::{
    backend::Backend,
    layout::{Constraint, Corner, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::components::ListComponent;
use crate::tabs::{HistoryTab, MainTab};

pub struct App {
    main_tab: MainTab,
    history_tab: HistoryTab,
    do_quit: bool,
    current_tab: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            main_tab: MainTab::new(),
            history_tab: HistoryTab::new(),
            do_quit: false,
            current_tab: 0,
        }
    }

    pub fn event(&mut self, ev: KeyEvent) {
        if ev == KeyEvent::from(KeyCode::Char('q')) {
            self.do_quit = true;
            return;
        }

        if ev == KeyEvent::from(KeyCode::Char('1')) {
            self.current_tab = 0;
        } else if ev == KeyEvent::from(KeyCode::Char('2')) {
            self.current_tab = 1;
        }

        if self.current_tab == 0 {
            self.main_tab.event(ev);
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

        match self.current_tab {
            0 => self.main_tab.draw(f, chunks[1]),
            1 => self.history_tab.draw(f, chunks[1]),
            _ => {}
        };

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
