pub fn is_aplha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

pub fn is_aplha_digit(c: char) -> bool {
    is_aplha(c) || c.is_ascii_digit()
}

pub fn format_string_to_f64(number: &String) -> String {
    let num = number.parse::<f64>().unwrap();
    let formatted_number = if num.fract() == 0.0 {
        format!("{:.1}", num)
    } else {
        format!("{}", num)
    };
    formatted_number
}
