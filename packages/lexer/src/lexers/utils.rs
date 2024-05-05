use crate::{chars::LexerChar, TokenPosition};

const NEWLINES: [&str; 3] = ["\n", "\r", "\r\n"];
const WHITESPACES: [&str; 3] = [" ", "	", "\t"];

pub fn is_newline(char: &str) -> bool {
    NEWLINES.contains(&char)
}

pub fn is_whitespace(char: &str) -> bool {
    WHITESPACES.contains(&char) || is_newline(char)
}

pub fn lookbehind_raw_token(
    current_position: &TokenPosition,
    buffer: &[String],
    stop_char: Option<LexerChar>,
) -> (Vec<String>, TokenPosition, TokenPosition) {
    println!("lookbehind_raw_token: buffer: {:#?}", buffer,);

    let last_identifier_char_index = buffer
        .iter()
        .rposition(|c| {
            !is_whitespace(c)
                && (stop_char.is_none() || *c != stop_char.as_ref().unwrap().to_string())
        })
        .unwrap_or(0);
    let trailing_trimmed = &buffer[..last_identifier_char_index + 1];
    let trimmed_buffer: Vec<String> = trailing_trimmed
        .iter()
        .map(|c| c.to_owned())
        .filter(|c| {
            !is_whitespace(c)
                && (stop_char.is_none() || *c != stop_char.as_ref().unwrap().to_string())
        })
        .collect();

    println!(
        "lookbehind_raw_token: trimmed_buffer: {:#?}",
        trimmed_buffer,
    );

    let to_col = current_position.column - (buffer.len() - (last_identifier_char_index + 1));

    let from = TokenPosition::new(
        current_position.line,
        to_col.saturating_sub(trimmed_buffer.len().saturating_sub(1)),
    );
    let to = TokenPosition::new(current_position.line, to_col);

    (trimmed_buffer, from, to)
}
