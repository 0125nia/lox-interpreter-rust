use super::handle;
use super::scanner::Scanner;
macro_rules! lexemes_methods {
    ($name:ident {
        $(
            $variant:ident => $ch:expr => $method:expr
        ),* $(,)?
    }) => {
        pub enum $name {
            $($variant),*
        }

        impl $name {
            pub fn from_char(c: char) -> Option<Self> {
                match c {
                    $(
                        $ch => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }

            pub fn execute(&self, scanner: &mut Scanner) {
                match self {
                    $(Self::$variant => $method(scanner)),*
                }
            }
        }
    };
}

lexemes_methods! {
    Lexemes {
        LeftParenthesis => '(' => |_| println!("LEFT_PAREN ( null"),
        RightParenthesis => ')' => |_| println!("RIGHT_PAREN ) null"),
        LeftBrace => '{' => |_| println!("LEFT_BRACE {} null",'{'),
        RightBrace => '}' => |_| println!("RIGHT_BRACE {} null",'}'),
        Star => '*' => |_| println!("STAR * null"),
        Dot => '.' => |_| println!("DOT . null"),
        Comma => ',' => |_| println!("COMMA , null"),
        Plus => '+' => |_| println!("PLUS + null"),
        Minus => '-' => |_| println!("MINUS - null"),
        Semicolon => ';' => |_| println!("SEMICOLON ; null"),
        Equal => '=' => |scanner| handle::equal(scanner),
        Bang => '!' => |scanner| handle::bang(scanner),
        Greater => '>' => |scanner| handle::greater(scanner),
        Less => '<' => |scanner| handle::less(scanner),
        Division => '/' => |scanner| handle::division(scanner),
    }
}
