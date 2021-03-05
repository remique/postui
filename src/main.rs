// use mybf::BfFile;
use mybf::*;

use std::io;
use std::io::Read;
use std::str;
use termion::{async_stdin, event::Key, input::TermRead, raw::IntoRawMode};
use tui::backend::TermionBackend;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Widget;
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;
use tui::Terminal;

trait Backend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>;
}

trait DrawaBleComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, rect: Rect) -> Result<()>;
}

struct RegCell<'a> {
    text: &'a str,
    block: Option<Block<'a>>,
    border_type: BorderType,
}

impl<'a> Default for RegCell<'a> {
    fn default() -> RegCell<'a> {
        RegCell {
            text: "",
            block: None,
            border_type: BorderType::Plain,
        }
    }
}

impl<'a> Widget for RegCell<'a> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let text_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        // let num_of_constraints: [Constraint; 10] = [
        //     Constraint::Percentage(10),
        //     Constraint::Percentage(10),
        //     Constraint::Percentage(10),
        //     Constraint::Percentage(10),
        //     Constraint::Percentage(10),
        //     Constraint::Percentage(10),
        //     Constraint::Percentage(10),
        //     Constraint::Percentage(10),
        //     Constraint::Percentage(10),
        //     Constraint::Percentage(10),
        // ];

        let mut num_of_constraints = Vec::new();
        for _ in 0..10 {
            num_of_constraints.push(Constraint::Percentage(10));
        }

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(num_of_constraints)
            .split(text_area);

        let myvec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let myvec_2 = vec!["", "", "", "", "^", "", "", "", "", ""];

        let mybl = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        for (i, x) in chunks.iter().enumerate() {
            mybl.clone().render(chunks[i], buf);
        }

        for (i, x) in myvec.iter().enumerate() {
            let needed_height = (chunks[i].bottom() - chunks[i].top()) / 2;
            let needed_width = (chunks[i].right() - chunks[i].left()) / 2;

            // buf.set_string(text_area.left() + i as u16 * 3, text_area.top(), format!("{}", &x), Style::default());
            buf.set_string(
                chunks[i].left() + needed_width,
                chunks[i].top() + needed_height,
                format!("{}", &x),
                Style::default(),
            );
        }

        for (i, x) in myvec_2.iter().enumerate() {
            let needed_height = (chunks[i].bottom() - chunks[i].top()) / 2;
            let needed_width = (chunks[i].right() - chunks[i].left()) / 2;
            buf.set_string(
                chunks[i].left() + needed_width,
                chunks[i].top() + needed_height + 1,
                format!("{}", &x),
                Style::default(),
            );
        }
    }
}

impl<'a> RegCell<'a> {
    fn text(mut self, text: &'a str) -> RegCell<'a> {
        self.text = text;
        self
    }

    fn block(mut self, block: Block<'a>) -> RegCell<'a> {
        self.block = Some(block);
        self
    }

    fn border_type(mut self, border_type: BorderType) -> RegCell<'a> {
        self.border_type = border_type;
        self
    }
}

fn main() -> Result<(), io::Error> {
    let mut file = BfFile::new("test.bf");
    println!("{}", file.tape.value_of_index(2));

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
                        Constraint::Percentage(20),
                        Constraint::Percentage(70),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(frame.size());

            let block = Block::default()
                .title("Some title")
                .borders(Borders::ALL)
                .style(Style::default());

            let block_2 = RegCell::default()
                .text("Test")
                .block(Block::default().borders(Borders::ALL))
                .border_type(BorderType::Plain);

            // The output should be fixed length no matter what,
            // which is 1 line + borders(top&down) + 1 space = 4 lines
            let block_3 = Block::default()
                .title("Output")
                .borders(Borders::ALL)
                .style(Style::default());

            let text = vec![Spans::from(Span::styled(
                "Second line",
                Style::default().fg(Color::Red),
            ))];

            let block_3 = Paragraph::new(text)
                // .block(Block::default().title("Paragraph").borders(Borders::NONE))
                .style(Style::default().fg(Color::White).bg(Color::Black));

            frame.render_widget(block_2, chunks[0]);
            frame.render_widget(block, chunks[1]);
            frame.render_widget(block_3, chunks[2]);
        })?;

        for k in async_input.by_ref().keys() {
            match k.unwrap() {
                Key::Char('q') => {
                    return Ok(());
                }
                Key::Char('w') => {
                    file.run();
                }
                _ => (),
            }
        }
    }
}
