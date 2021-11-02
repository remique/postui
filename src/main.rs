#![allow(dead_code)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

mod foldertree;

use crate::foldertree::*;

struct AppState {
    count: i32,
}

impl AppState {
    fn new() -> Self {
        AppState { count: 0 }
    }
}

fn do_thread_stuff(app_ref: &Arc<Mutex<AppState>>, rx: &std::sync::mpsc::Receiver<i32>) {
    // rx.try_recv().unwrap();
    // let mut locked = app_ref.lock().unwrap();
    // locked.count += 1;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MyEndpoint {
    r#type: String,
    name: String,
    method: String,
}

#[derive(Debug)]
pub struct MyFolder {
    r#type: String,
    name: String,
    folded: bool,
    items: Vec<MyEndpoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyEndpointOrFolder {
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#folded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#items: Option<Vec<MyEndpoint>>,
}

impl MyEndpointOrFolder {
    fn from_endpoint(e: MyEndpoint) -> MyEndpointOrFolder {
        MyEndpointOrFolder {
            r#type: Some(e.r#type),
            name: Some(e.name),
            method: Some(e.method),
            folded: None,
            items: None,
        }
    }

    fn from_folder(f: MyFolder) -> MyEndpointOrFolder {
        MyEndpointOrFolder {
            r#type: Some(f.r#type),
            name: Some(f.name),
            method: None,
            folded: Some(f.folded),
            items: Some(f.items),
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Set up terminal output
    // let stdout = io::stdout().into_raw_mode()?;
    // let backend = TermionBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;

    // // Create a separate thread to poll stdin.
    // // This provides non-blocking input support.
    // let mut asi = async_stdin();

    // let app = Arc::new(Mutex::new(AppState::new()));
    // let cloned_app = Arc::clone(&app);
    // let (tx, rx) = std::sync::mpsc::channel();

    // thread::spawn(move || loop {
    //     do_thread_stuff(&app, &rx);
    //     // thread::sleep(time::Duration::from_millis(100));
    // });
    let j = r#"
[
    {
        "type": "folder",
        "name": "Pierwszy",
        "folded": false,
        "items": [
            {
                "type": "endpoint",
                "name": "Dodaj usera",
                "method": "POST"
            },
            {
                "type": "endpoint",
                "name": "Zmien userow",
                "method": "PUT"
            },
            {
                "type": "folder",
                "name": "Nested",
                "folded": false,
                "items": [
                    {
                        "type": "endpoint",
                        "name": "Nested jeszcze",
                        "method": "GET"
                    }
                ]
            }
        ]
    },
    {
        "type": "endpoint",
        "name": "Costam",
        "method": "POST"
    }
]
    "#;

    let k = r#"
[
    {
        "type": "folder",
        "name": "Pierwszy",
        "folded": false,
        "items": [
            {
                "type": "endpoint",
                "name": "Dodaj usera",
                "method": "POST"
            },
            {
                "type": "endpoint",
                "name": "Zmien userow",
                "method": "PUT"
            }
        ]
    },
    {
        "type": "endpoint",
        "name": "Costam",
        "method": "POST"
    }
]
    "#;

    // let p: Vec<MyEndpointOrFolder> = serde_json::from_str(&k).expect("xd");
    // println!("{:#?}", p);

    let mut ftc = FolderTreeComponent::new();
    ftc.from_str(j);

    for item in ftc.items {
        println!("{:#?}", item.obj);
    }

    Ok(())

    // // Clear the terminal before first draw.
    // terminal.clear()?;
    // loop {
    //     let mut app = cloned_app.lock().unwrap();

    //     // Lock the terminal and start a drawing session.
    //     terminal.draw(|frame| {
    //         // Create a layout into which to place our blocks.
    //         let chunks = Layout::default()
    //             .direction(Direction::Horizontal)
    //             .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
    //             .split(frame.size());

    //         // Create a block...
    //         let block = Block::default()
    //             // With a given title...
    //             .title("Color Changer")
    //             // Borders on every side...
    //             .borders(Borders::ALL)
    //             // The background of the current color...
    //             .style(Style::default());

    //         // Render into the first chunk of the layout.
    //         frame.render_widget(block, chunks[0]);

    //         // The text lines for our text box.
    //         let txt = vec![Spans::from(format!("{}", app.count))];
    //         // Create a paragraph with the above text...
    //         let graph = Paragraph::new(txt)
    //             // In a block with borders and the given title...
    //             .block(Block::default().title("Text box").borders(Borders::ALL))
    //             // With white foreground and black background...
    //             .style(Style::default().fg(Color::White).bg(Color::Black));

    //         // Render into the second chunk of the layout.
    //         frame.render_widget(graph, chunks[1]);
    //     })?;

    //     // Iterate over all the keys that have been pressed since the
    //     // last time we checked.
    //     for k in asi.by_ref().keys() {
    //         match k.unwrap() {
    //             // If any of them is q, quit
    //             Key::Char('q') => {
    //                 // Clear the terminal before exit so as not to leave
    //                 // a mess.
    //                 terminal.clear()?;
    //                 return Ok(());
    //             }
    //             Key::Char('e') => {
    //                 tx.send(10).unwrap();
    //             }
    //             // Otherwise, throw them away.
    //             _ => (),
    //         }
    //     }
    // }
}
