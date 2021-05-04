use std::fs;

pub struct Tape {
    pub tape: Vec<i32>,
    pos: usize,
}

impl Tape {
    pub fn new() -> Tape {
        let tape = vec![0; 1024];
        let pos = 0;

        Tape { tape, pos }
    }

    pub fn print_to(&self, to: usize) {
        println!("{:?}", &self.tape[0..to]);
    }

    pub fn add_one(&mut self) {
        self.tape[self.pos] = self.tape[self.pos] + 1;
    }

    pub fn sub_one(&mut self) {
        self.tape[self.pos] = self.tape[self.pos] - 1;
    }

    pub fn move_right(&mut self) {
        self.pos = self.pos + 1;
    }

    pub fn move_left(&mut self) {
        self.pos = self.pos - 1;
    }

    pub fn curr_pos_value(&self) -> u8 {
        self.tape[self.pos] as u8
    }

    pub fn value_of_index(&self, idx: usize) -> u8 {
        self.tape[idx] as u8
    }

    pub fn tape_to(&self, to: usize) -> &[i32] {
        &self.tape[0..to]
    }
}

pub struct BfFile {
    filename: String,
    pos: usize,
    pub chars: Vec<u8>,
    pub tape: Tape,
}

impl BfFile {
    pub fn new(name: &str) -> BfFile {
        let filename = name.to_string();
        let pos = 0;
        let chars = fs::read(name).expect("Unable to read the file");
        let tape = Tape::new();

        let chars = fs::read(name)
            .expect("Blabla")
            .iter()
            .filter(|x| **x != 10 as u8)
            .map(|x| *x)
            .collect::<Vec<_>>();

        BfFile {
            filename,
            pos,
            chars,
            tape,
        }
    }

    pub fn run(&mut self) {
        let reg = &mut self.tape;

        loop {
            let cur_c = self.chars[self.pos];

            print!("{}", self.chars[self.pos] as char);

            match cur_c as char {
                '+' => reg.add_one(),
                '-' => reg.sub_one(),
                '<' => reg.move_left(),
                '>' => reg.move_right(),
                '[' => {
                    if reg.curr_pos_value() == 0 {
                        // skip to ']'
                        while self.chars[self.pos] as char != ']' {
                            self.pos = self.pos + 1;
                        }
                    }
                }
                ']' => {
                    if reg.curr_pos_value() != 0 {
                        // jump back to '['
                        while self.chars[self.pos] as char != '[' {
                            self.pos = self.pos - 1;
                        }
                    }
                }
                '.' => (),
                // '.' => print!("{}", reg.curr_pos_value() as char),
                _ => (),
            }

            self.pos = self.pos + 1;

            if self.pos >= self.chars.len() - 1 {
                break;
            }
        }
    }

    pub fn next(&mut self) {
        let reg = &mut self.tape;

        let cur_c = self.chars[self.pos];
        match cur_c as char {
            '+' => reg.add_one(),
            '-' => reg.sub_one(),
            '<' => reg.move_left(),
            '>' => reg.move_right(),
            '[' => {
                if reg.curr_pos_value() == 0 {
                    // skip to ']'
                    while self.chars[self.pos] as char != ']' {
                        self.pos = self.pos + 1;
                    }
                }
            }
            ']' => {
                if reg.curr_pos_value() != 0 {
                    // jump back to '['
                    while self.chars[self.pos] as char != '[' {
                        self.pos = self.pos - 1;
                    }
                }
            }
            '.' => (),
            _ => {}
        }

        self.pos = self.pos + 1;

        // TODO: Do it better lol
        let next_c = self.chars[self.pos];
        match next_c as char {
            '+' | '-' | '<' | '>' | ']' | '[' | '.' => {}
            _ => {
                self.next();
            }
        }

        if self.pos >= self.chars.len() {
            self.pos = 0;
        }
    }

    pub fn current_pos(&self) -> usize {
        self.pos
    }

    pub fn current_char(&self) -> char {
        self.chars[self.pos] as char
    }
}
