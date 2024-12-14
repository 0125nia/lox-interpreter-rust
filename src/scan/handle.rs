use crate::pkg::{code::ExitCode, util};

use super::scanner::Scanner;

macro_rules! lexemes_handle {
    ($name:ident, $token_name:expr, $lexeme:expr, $follow_token_name:expr, $follow_lexeme:expr) => {
        pub fn $name(scanner: &mut Scanner) {
            if let Some($follow_lexeme) = scanner.chars.peek() {
                println!("{} {}{} null", $follow_token_name, $lexeme, $follow_lexeme);
                scanner.chars.next();
            } else {
                println!("{} {} null", $token_name, $lexeme);
            }
        }
    };
}

lexemes_handle!(equal, "EQUAL", '=', "EQUAL_EQUAL", '=');
lexemes_handle!(bang, "BANG", '!', "BANG_EQUAL", '=');
lexemes_handle!(greater, "GREATER", '>', "GREATER_EQUAL", '=');
lexemes_handle!(less, "LESS", '<', "LESS_EQUAL", '=');

pub fn division(scanner: &mut Scanner) {
    if let Some('/') = scanner.chars.peek() {
        scanner.chars.next();
        while let Some(&ch) = scanner.chars.peek() {
            if ch == '\n' {
                break;
            }
            scanner.chars.next();
        }
        return;
    }
    println!("SLASH / null");
}

pub fn quotation(scanner: &mut Scanner) {
    let mut string_content = String::new();
    let mut flag = false;
    while let Some(ch) = scanner.chars.next() {
        if ch == '"' {
            flag = true;
            break;
        }
        if let Some('\n') = scanner.chars.peek() {
            break;
        }
        string_content.push(ch);
    }
    if flag {
        println!("STRING \"{}\" {}", string_content, string_content);
    } else {
        eprintln!("[line {}] Error: Unterminated string.", scanner.line_num);
        scanner.exit_code = ExitCode::Exit;
    }
}

pub fn number(scanner: &mut Scanner, c: char) {
    let mut number = String::new();
    number.push(c);
    while let Some(&ch) = scanner.chars.peek() {
        if ch.is_ascii_digit() || ch == '.' {
            number.push(ch);
            scanner.chars.next();
        } else {
            break;
        }
    }
    let formatted_number = util::format_string_to_f64(&number);
    println!("NUMBER {} {}", number, formatted_number);
}

pub fn identifier(scanner: &mut Scanner, c: char) {
    let mut identifier = String::new();
    identifier.push(c);
    while let Some(&ch) = scanner.chars.peek() {
        if util::is_aplha_digit(ch) {
            identifier.push(ch);
            scanner.chars.next();
        } else {
            break;
        }
    }
    println!("IDENTIFIER {} null", identifier);
}
