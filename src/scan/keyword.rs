use strum_macros::EnumString;

#[derive(Debug, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Keyword {
    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}
