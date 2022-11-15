use crate::tokenize_error;
use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::{util as util, TokenErrorType, TokenizeError};

pub fn is_whitespace(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_whitespace(token)
}

pub fn consume_whitespace(tokenizer: &mut Tokenizer) -> Result<(), TokenizeError>  {
    if !is_whitespace(tokenizer) {
        return tokenize_error!(TokenErrorType::UnexpectedToken, tokenizer);
    }

    tokenizer.consume().unwrap();

    return Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenErrorType};

    #[test]
    fn is_whitespace_input_whitespace() {
        let input = String::from_str(" ").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_whitespace(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_text_input_not_whitespace() {
        let input = String::from_str("Foobar").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_whitespace(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn consume_whitespace_input() {
        let input = String::from_str(" ").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        _ = super::consume_whitespace(&mut tokenizer).unwrap();

        assert_eq!(tokenizer.get_current_index(), input.len());
    }

    #[test]
    fn consume_invalid_input() {
        let input = String::from_str("🦀").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_whitespace(&mut tokenizer).unwrap_err();

        assert_eq!(token.error_type, TokenErrorType::UnexpectedToken);
    }

}
