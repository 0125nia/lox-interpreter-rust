use std::{iter::Peekable, str::Chars};
use crate::pkg::code::ExitCode;

use crate::scan::lexemes::Lexemes;

pub struct Scanner<'a> {
    pub chars: Peekable<Chars<'a>>,
    line_num: i32,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            line_num: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> ExitCode {
        let mut exit_code = ExitCode::Continue;
        while let Some(c) = self.chars.next() {
            if c == '\n' {
                self.line_num += 1;
                continue;
            }
            if let Some(l) = Lexemes::from_char(c) {
                l.execute(self);
            } else {
                eprintln!("[line {}] Error: Unexpected character: {}", self.line_num + 1, c);
                exit_code = ExitCode::Exit
            }
        }
        println!("EOF  null");
        exit_code
    }
}