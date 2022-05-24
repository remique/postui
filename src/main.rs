#![allow(dead_code)]
#![allow(unused_imports)]
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::stdin;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{thread, time};
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use tui::{backend::CrosstermBackend, Terminal};

mod app;
mod components;
mod foldertree;
mod tabs;

use crate::app::*;
use crate::foldertree::*;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;

    // Set up terminal output
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    let mut app = App::new();

    // Clear the terminal before first draw.
    terminal.clear()?;

    loop {
        draw(&mut terminal, &mut app)?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(event) = event::read()? {
                app.event(event);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if app.is_quit() {
            break;
        }
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}

fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    terminal.draw(|frame| {
        if let Err(e) = app.draw(frame) {
            panic!("failed to draw the app: {}", e);
        }
    })?;

    Ok(())
}
