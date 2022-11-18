use once_cell::sync::Lazy;

use crate::{tokenizer::{tokenizer::Tokenizer, TokenErrorType, Token, TokenizeError, TokenType, FileLocation}, tokenize_error};

use super::is_word;

static KEYWORDS: Lazy<Vec<&str>> = Lazy::new(|| {
    vec![
        "break", "do", "instanceof", "typeof",
        "case", "else", "new", "var", "catch",
        "finally", "return", "void", "continue",
        "for", "switch", "while", "debugger",
        "function", "this", "with", "default",
        "if", "throw", "delete", "in", "try"
    ]
});

pub fn is_keyword(tokenizer: &Tokenizer, keyword: &str) -> bool {
    is_word(tokenizer, keyword)
}

pub fn find_keyword(tokenizer: &Tokenizer) -> Result<&'static str, ()> {
    for word in KEYWORDS.iter() {
        if is_word(tokenizer, *word) { return Ok(*word) }
    }

    return Err(())
}

pub fn consume_keyword(tokenizer: &mut Tokenizer, keyword: &str) -> Result<Token, TokenizeError> {
    if !is_keyword(tokenizer, keyword) { return tokenize_error!(TokenErrorType::UnexpectedToken, tokenizer); }
    let start = tokenizer.get_current_index();
    let start_pos = tokenizer.get_current_file_loc();
    let mut raw_value = String::new();

    for _ in 0..keyword.len() {
        raw_value.push(tokenizer
            .consume()
            .unwrap()
            .clone()
        );
    }

    let end  = tokenizer.get_current_index();
    let end_pos = tokenizer.get_current_file_loc();
    let value = raw_value.clone();

    Ok(Token {
        token_type: TokenType::Keyword,
        value,
        raw_value,
        range: (start, end),
        loc: FileLocation { start: start_pos, end: end_pos }
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenType};

    #[test]
    fn if_is_if_is_true() {
        let input = String::from_str("if").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_keyword(&tokenizer, "if");

        assert_eq!(result, true);
    }

    #[test]
    fn while_is_if_is_false() {
        let input = String::from_str("while").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_keyword(&tokenizer, "if");

        assert_eq!(result, false);
    }

    #[test]
    fn find_if_keyword() {
        let input = String::from_str("if").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::find_keyword(&tokenizer).unwrap();

        assert_eq!(result, "if");
    }

    #[test]
    fn consume_if_keyword() {
        let input = String::from_str("if").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let result = super::consume_keyword(&mut tokenizer, "if").unwrap();

        assert_eq!(result.token_type, TokenType::Keyword);
        assert_eq!(result.value, "if");
        assert_eq!(result.raw_value, "if");
        assert_eq!(tokenizer.get_current_index(), 2);
    }

}
