use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use codecrafters_interpreter::scan::lexemes::Lexemes;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let mut exit_code = 0;

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            // Uncomment this block to pass the first stage
            if !file_contents.is_empty() {
                file_contents.lines().enumerate().for_each(|(line_num,line)| for c in line.chars() {
                    if let Some(l) = Lexemes::from_char(c) {
                        l.execute();
                    }else {
                        eprintln!("[line {}] Error: Unexpected character: {}", line_num+1, c);
                        exit_code = 65;
                    }
                });
                println!("EOF  null");

                if exit_code != 0 {
                    exit(exit_code)
                }
                return;
            } else {
                println!("EOF  null");
                // Placeholder, remove this line when implementing the scanner
            }
            
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
    
}
