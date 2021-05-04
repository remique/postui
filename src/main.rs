// use mybf::BfFile;
use mybf::*;

use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::str;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::Text;
use tui::text::{Span, Spans};
use tui::widgets::Widget;
use tui::widgets::{Block, Borders, Paragraph, Tabs};
use tui::Terminal;

struct TapeComponent<'a> {
    // Block to wrap the widget with
    block: Option<Block<'a>>,
    // Lines compressed into a vector of strings (easier to iterate)
    bf_file: &'a BfFile,
    // Style for the widget
    style: Style,
}

impl<'a> TapeComponent<'a> {
    fn new(bf_file: &'a BfFile) -> TapeComponent<'a> {
        TapeComponent {
            block: None,
            bf_file,
            style: Default::default(),
        }
    }
    fn read_file(mut self, bf_file: &'a BfFile) -> TapeComponent<'a> {
        self.bf_file = bf_file;
        self
    }
    pub fn block(mut self, block: Block<'a>) -> TapeComponent<'a> {
        self.block = Some(block);
        self
    }
    pub fn style(mut self, style: Style) -> TapeComponent<'a> {
        self.style = style;
        self
    }
}

impl<'a> Widget for TapeComponent<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, self.style);
        let tabs_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        for (i, j) in self.bf_file.tape.tape_to(10).iter().enumerate() {
            let space = i * 6;
            buf.set_string(
                tabs_area.left() + (space as u16),
                tabs_area.top(),
                j.to_string(),
                Style::default(),
            );
        }
    }
}

fn prepare_text<'a>(pos: usize, file: &BfFile) -> Vec<Spans<'a>> {
    let mut to_return: Vec<Spans> = Vec::new();
    let contents = fs::read_to_string("test2.bf").expect("blabla");
    let mut comment_style = Style::default().fg(Color::Rgb(8, 8, 8));

    let mut normal_style = Style::default();

    let mut counter: usize = 0;

    if counter >= file.chars.len() - 1 {
        counter = 0;
    }

    for line in contents.lines() {
        let mut inner_vec: Vec<Span> = Vec::new();
        for line_char in line.chars() {
            if counter == pos {
                normal_style = Style::default().bg(Color::White);
                comment_style = Style::default().bg(Color::White).fg(Color::Rgb(8, 8, 8));
            } else {
                normal_style = Style::default();
                comment_style = Style::default().fg(Color::Rgb(8, 8, 8));
            }
            match line_char {
                '+' | '-' | '<' | '>' | '[' | ']' | '.' => {
                    inner_vec.push(Span::styled(line_char.to_string(), normal_style));
                }
                _ => {
                    inner_vec.push(Span::styled(line_char.to_string(), comment_style));
                }
            }
            counter = counter + 1;
        }
        to_return.push(Spans::from(inner_vec.clone()));
    }

    to_return
}

fn main() -> Result<(), io::Error> {
    let mut file = BfFile::new("test2.bf");
    // println!("{}", file.tape.value_of_index(2));

    // Set up backend and terminal stuff
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut async_input = async_stdin();

    terminal.clear()?;
    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(4),
                        Constraint::Min(2),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            let titles = ["Run", "Hexdump", "Settings"]
                .iter()
                .cloned()
                .map(Spans::from)
                .collect();

            let tabs_component = Tabs::new(titles)
                .block(Block::default().title("Tabs").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow));

            let tape_component = TapeComponent::new(&file)
                .read_file(&file)
                .block(Block::default().borders(Borders::ALL));

            let my_text = prepare_text(file.current_pos(), &file);

            let block = Paragraph::new(my_text.clone())
                .block(Block::default().title("Code").borders(Borders::ALL))
                .style(Style::default());

            frame.render_widget(tabs_component, chunks[0]);
            frame.render_widget(tape_component, chunks[1]);
            frame.render_widget(block, chunks[2]);
        })?;

        for k in async_input.by_ref().keys() {
            match k.unwrap() {
                Key::Char('q') => {
                    return Ok(());
                }
                Key::Char('w') => {
                    // TODO: tutaj sprawdzamy czy nie wyjechalismy poza buffer
                    // if current_pos < max_pos --> file.next()
                    file.next();
                }
                _ => (),
            }
        }
    }
}

// fn main() {
//     let mut file = BfFile::new("test.bf");
//     file.run();
// }
