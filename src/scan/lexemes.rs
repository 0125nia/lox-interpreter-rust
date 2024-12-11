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

            pub fn execute(&self) {
                match self {
                    $(Self::$variant => $method),*
                }
            }
        }
    };
}

lexemes_methods! {
    Lexemes {
        LeftParenthesis => '(' => println!("LEFT_PAREN ( null"),
        RightParenthesis => ')' => println!("RIGHT_PAREN ) null"),
        LeftBrace => '{' => println!("LEFT_BRACE {} null",'{'),
        RightBrace => '}' => println!("RIGHT_BRACE {} null",'}'),
    }

}
