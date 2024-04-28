use unicode_segmentation::UnicodeSegmentation;

// TODO: we dont really need this package, just move this to lexer package's tests mod
pub fn str_to_graphemes(input: &str) -> Vec<String> {
    input
        .trim()
        .graphemes(true)
        .map(|char| char.to_owned())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::str_to_graphemes;

    #[test]
    fn converts_str_to_grapheme_vector() {
        let input = "
hello 😍 我 name is a̐ é ö̲
wow
cool
";

        let output = str_to_graphemes(input);

        assert_eq!(output.len(), 32);
        insta::assert_debug_snapshot!(output);
    }
}
