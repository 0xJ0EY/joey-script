use crate::{tokenize_error, tokenizer::{Token, tokenizer::Tokenizer, TokenizeError, TokenErrorType, Literal, TokenType}};

use super::is_word;

pub fn is_null(tokenizer: &Tokenizer) -> bool {
    is_word(tokenizer, "null")
}

pub fn consume_null(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError> {
    if !is_null(tokenizer) { return tokenize_error!(TokenErrorType::UnexpectedToken, tokenizer); }

    let start = tokenizer.get_current_index();

    let mut raw_value = String::new();

    for _ in 0..4 {
        raw_value.push(tokenizer
            .consume()
            .unwrap()
            .clone()
        );
    }

    let end  = tokenizer.get_current_index();

    Ok(Token {
        token_type: TokenType::Literal(Literal::Null),
        value: String::new(),
        raw_value,
        range: (start, end),
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenErrorType, TokenType, Literal};

    #[test]
    fn null_is_null() {
        let input = String::from_str("null").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_null(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn foobar_is_not_null() {
        let input = String::from_str("foobar").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_null(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn consume_open_null_input() {
        let input = String::from_str("null").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_null(&mut tokenizer).unwrap();

        assert_eq!(tokenizer.get_current_index(), input.len());
        assert_eq!(token.token_type, TokenType::Literal(Literal::Null));
        assert_eq!(token.value, "");
        assert_eq!(token.raw_value, "null");
        assert_eq!(tokenizer.get_current_index(), 4)
    }

    #[test]
    fn consume_invalid_input() {
        let input = String::from_str("🦀").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_null(&mut tokenizer).unwrap_err();

        assert_eq!(token.error_type, TokenErrorType::UnexpectedToken);
    }

}

