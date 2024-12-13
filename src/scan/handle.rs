use super::scanner::Scanner;


macro_rules! lexemes_handle {
    ($name:ident, $token_name:expr, $lexeme:expr, 
        $follow_token_name:expr, $follow_lexeme:expr) => {
        pub fn $name(scanner: &mut Scanner) {
            if let Some($follow_lexeme) = scanner.chars.peek() {
                println!("{} {}{} null", $follow_token_name, $lexeme,$follow_lexeme);
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
