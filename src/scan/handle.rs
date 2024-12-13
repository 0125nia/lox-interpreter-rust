use super::scanner::Scanner;


pub fn handle_equal(scanner: &mut Scanner) {
    if let Some('=') = scanner.chars.peek() {
        println!("EQUAL_EQUAL == null");
        scanner.chars.next();
    } else {
        println!("EQUAL = null");
    }
}

pub fn handle_bang(scanner: &mut Scanner) {
    if let Some('=') = scanner.chars.peek() {
        println!("BANG_EQUAL != null");
        scanner.chars.next();
    } else {
        println!("BANG ! null");
    }
}