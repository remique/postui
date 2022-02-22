#![allow(dead_code)]
#![allow(unused_imports)]
use crossterm::event::{self, Event, KeyCode};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::stdin;
use std::io::Read;
use std::sync::{Arc, Mutex};
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

use crate::app::*;
use crate::foldertree::*;

fn main() -> Result<(), io::Error> {
    // Set up terminal output
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();

    // Clear the terminal before first draw.
    terminal.clear()?;
    loop {
        draw(&mut terminal, &app)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> io::Result<()> {
    terminal.draw(|frame| {
        if let Err(e) = app.draw(frame) {
            panic!("failed to draw the app: {}", e);
        }
    })?;

    Ok(())
}
