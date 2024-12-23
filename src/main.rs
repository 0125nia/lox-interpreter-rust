use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use codecrafters_interpreter::pkg::code::ExitCode;
use codecrafters_interpreter::scan::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let mut scanner = Scanner::new(&file_contents);

            if let ExitCode::Exit = scanner.scan_tokens() {
                exit(ExitCode::Exit.into());
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
