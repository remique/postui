use crossterm::event::{KeyCode, KeyEvent};
use serde_json::{to_string, Map, Value};
use std::error::Error;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::components::{CommandType, FolderPopup, ListComponent, MainPaneComponent};

pub struct MainTab<'a> {
    list_component: ListComponent,
    main_pane: MainPaneComponent<'a>,
    folder_popup: FolderPopup<'a>,
    focus: Focus,
    pub current_cmds: Vec<CommandType>,
}

#[derive(PartialEq)]
enum Focus {
    FolderTreeWindow,
    MainPane, // This will be changed later on
    FolderPopup,
}

struct Request {
    method: String,
    url: String,
    // TODO: Should this be String or Value?
    json_body: Value,
}

impl MainTab<'_> {
    pub fn new() -> Self {
        let list_component = ListComponent::new("./config.json");
        let current_cmds = list_component.generate_cmds();

        Self {
            list_component,
            current_cmds,
            folder_popup: FolderPopup::new(),
            main_pane: MainPaneComponent::new(),
            focus: Focus::MainPane,
        }
    }

    pub fn event(&mut self, ev: KeyEvent) {
        // TODO: This shit needs refactor
        match self.focus {
            Focus::FolderTreeWindow => {
                let can_unfold = self.list_component.tree().can_unfold_folder();

                if ev.code == KeyCode::Right && !can_unfold {
                    self.switch_focus(Focus::MainPane);
                    self.current_cmds = self.main_pane.generate_cmds();
                } else {
                    self.list_component.event(ev);

                    if let Some(curr) = self.list_component.tree().get_current_endpoint() {
                        self.main_pane.current_endpoint = curr;
                    }
                }
                if ev.code == KeyCode::Char('a') {
                    self.folder_popup.is_open = !self.folder_popup.is_open;
                    self.switch_focus(Focus::FolderPopup);
                }
            }
            Focus::MainPane => {
                if ev.code == KeyCode::Left {
                    self.switch_focus(Focus::FolderTreeWindow);
                    self.current_cmds = self.list_component.generate_cmds();
                }
                if ev.code == KeyCode::Char('s') {
                    let endpoint = self.list_component.tree().get_current_endpoint();
                    let endpoint = match endpoint {
                        Some(e) => e,
                        None => Map::new(),
                    };

                    let request = prepare_request(endpoint);

                    tokio::spawn(async move {
                        let query = query_request(request).await;
                        match query {
                            Ok(r) => {
                                log::info!("{}", r);
                            }
                            Err(e) => {
                                log::error!("{}", e);
                            }
                        }
                    });
                }
            }
            Focus::FolderPopup => {
                if ev.code == KeyCode::Esc {
                    self.folder_popup.close();
                    self.switch_focus(Focus::FolderTreeWindow);
                }

                self.folder_popup.event(ev);

                if self.folder_popup.is_saved() == true {
                    self.folder_popup.close();
                    self.switch_focus(Focus::FolderTreeWindow);

                    self.list_component.tree().insert_endpoint();
                }
            }
        };
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>, r: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(r);

        self.list_component.draw(f, chunks[0]);
        self.main_pane.draw(f, chunks[1]);

        let centered = self.folder_popup.centered_rect(60, 60, f.size());
        self.folder_popup.draw(f, centered);
    }

    fn switch_focus(&mut self, f: Focus) {
        match f {
            Focus::FolderTreeWindow => {
                self.list_component.set_focus(true);
                self.main_pane.focused = false;
            }
            Focus::MainPane => {
                self.list_component.set_focus(false);
                self.main_pane.focused = true;
            }
            Focus::FolderPopup => {
                self.list_component.set_focus(false);
                self.main_pane.focused = false;
            }
        }

        self.focus = f;
    }
}

fn prepare_request(input: Map<String, Value>) -> Request {
    let empty = || String::from("");
    let get_val = |val: &Value| to_string(val).unwrap();

    let url = input
        .get("url")
        .map_or_else(empty, get_val)
        .replace("\"", "");
    let method = input
        .get("method")
        .map_or_else(empty, get_val)
        .replace("\"", "");

    let json_body = input.get("json_body").map_or_else(empty, get_val);

    let json_body = serde_json::from_str(json_body.as_str()).unwrap();

    Request {
        url,
        method,
        json_body,
    }
}

async fn query_request(input: Request) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let request = match input.method.as_str() {
        "POST" => client.post(input.url),
        "GET" => client.get(input.url),
        "PUT" => client.put(input.url),
        "DELETE" => client.delete(input.url),
        _ => return Err("No method found")?,
    };

    // let response = request.json(&input.json_body).send().await?.json().await?;
    let response = request.send().await?;

    Ok(format!("{:#?}", response.status()))
}
