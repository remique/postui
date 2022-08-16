use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::components::CommandComponent;
use crate::tabs::{DebugTab, HistoryTab, MainTab};

pub struct App<'a> {
    main_tab: MainTab<'a>,
    history_tab: HistoryTab,
    debug_tab: DebugTab,
    cmdbar: CommandComponent,
    do_quit: bool,
    current_tab: usize,
}

impl App<'_> {
    pub fn new() -> Self {
        let main_tab = MainTab::new();
        let history_tab = HistoryTab::new();
        let debug_tab = DebugTab::new();
        let cmdbar = CommandComponent::new(main_tab.current_cmds.clone());

        Self {
            main_tab,
            history_tab,
            debug_tab,
            cmdbar,
            do_quit: false,
            current_tab: 0,
        }
    }

    pub fn event(&mut self, ev: KeyEvent) {
        match ev {
            // Quit by hitting 'q' or 'ctrl-c'
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
            }
            | KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            } => {
                self.do_quit = true;
                return;
            }
            // Change to Tab 1
            KeyEvent {
                code: KeyCode::Char('1'),
                modifiers: KeyModifiers::NONE,
            } => {
                self.current_tab = 0;
                self.cmdbar.update_cmd(0);
            }
            // Change to Tab 2
            KeyEvent {
                code: KeyCode::Char('2'),
                modifiers: KeyModifiers::NONE,
            } => {
                self.current_tab = 1;
                self.cmdbar.update_cmd(1);
            }
            // Change to Tab 4
            KeyEvent {
                code: KeyCode::Char('4'),
                modifiers: KeyModifiers::NONE,
            } => {
                self.current_tab = 3;
                self.cmdbar.update_cmd(1);
            }
            _ => {}
        };

        if self.current_tab == 0 {
            self.main_tab.event(ev);
            self.cmdbar.cmds_from(self.main_tab.current_cmds.clone());
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
            0 => {
                self.main_tab.draw(f, chunks[1]);
                self.cmdbar.draw(f, chunks[2]);
            }
            1 => {
                self.history_tab.draw(f, chunks[1]);
                self.cmdbar.draw(f, chunks[2]);
            }
            3 => {
                self.debug_tab.draw(f, chunks[1]);
            }
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
            String::from("Debug"),
        ];

        let titles = titles
            .iter()
            .map(|item| Spans::from(vec![Span::styled(item, Style::default())]))
            .collect();

        let tabs = Tabs::new(titles)
            .select(self.current_tab)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        f.render_widget(tabs, r);
    }
}
