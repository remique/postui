#![allow(dead_code)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::io;
use std::io::stdin;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::Backend;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use tui::Terminal;

mod app;
mod components;
mod foldertree;

use crate::app::*;
use crate::foldertree::*;

fn main() -> Result<(), io::Error> {
    // Set up terminal output
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create a separate thread to poll stdin.
    // This provides non-blocking input support.
    let mut asi = async_stdin();

    let k = r#"
    {
        "root": [
            {
                "type": "folder",
                "name": "Pierwszy",
                "folded": false,
                "path": "/root/0",
                "items": [
                    {
                        "type": "endpoint",
                        "name": "Dodaj usera",
                        "method": "POST",
                        "path": "/root/0/items/0"
                    },
                    {
                        "type": "endpoint",
                        "name": "Zmien userow",
                        "method": "PUT",
                        "path": "/root/0/items/1"
                    },
                    {
                        "type": "folder",
                        "name": "Nested",
                        "folded": false,
                        "path": "/root/0/items/2",
                        "items": [
                            {
                                "type": "endpoint",
                                "name": "Nested jeszcze",
                                "method": "GET",
                                "path": "/root/0/items/2/items/0"
                            }
                        ]
                    }
                ]
            },
            {
                "type": "endpoint",
                "name": "Costam",
                "method": "POST",
                "path": "/root/1"
            },
            {
                "type": "folder",
                "name": "Trzeci folder",
                "folded": false,
                "path": "/root/2",
                "items": [
                    {
                        "type": "endpoint",
                        "name": "Nested jeszcze",
                        "method": "GET",
                        "path": "/root/2/items/0"
                    }
                ]
            }
        ]
    }
    "#;

    let app = App::new();

    // Clear the terminal before first draw.
    terminal.clear()?;
    loop {
        draw(&mut terminal, &app)?;

        for c in stdin().keys() {
            match c? {
                Key::Char('q') => {
                    return Ok(());
                }
                _ => {}
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
