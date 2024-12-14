use std::{iter::Peekable, str::Chars};
use crate::pkg::code::ExitCode;

use crate::scan::lexemes::Lexemes;

pub struct Scanner<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub line_num: i32,
    pub exit_code: ExitCode,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            line_num: 1,
            exit_code: ExitCode::Continue,
        }
    }

    pub fn scan_tokens(&mut self) -> ExitCode {
        while let Some(c) = self.chars.next() {
            if c == '\n' {
                self.line_num += 1;
                continue;
            }
            if let Some(l) = Lexemes::from_char(c) {
                l.execute(self);
            } else {
                eprintln!("[line {}] Error: Unexpected character: {}", self.line_num, c);
                self.exit_code = ExitCode::Exit;
            }
        }
        println!("EOF  null");
        self.exit_code.clone()
    }
}