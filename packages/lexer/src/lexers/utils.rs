const NEWLINES: [&str; 3] = ["\n", "\r", "\r\n"];
const WHITESPACES: [&str; 3] = [" ", "	", "\t"];

pub fn is_newline(char: &str) -> bool {
    return NEWLINES.contains(&char);
}

pub fn is_whitespace(char: &str) -> bool {
    return WHITESPACES.contains(&char) || is_newline(char);
}
