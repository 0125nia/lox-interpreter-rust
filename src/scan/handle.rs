use super::scanner::Scanner;


pub fn equal(scanner: &mut Scanner) {
    if let Some('=') = scanner.chars.peek() {
        println!("EQUAL_EQUAL == null");
        scanner.chars.next();
    } else {
        println!("EQUAL = null");
    }
}