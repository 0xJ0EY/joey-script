use crate::tokenize_error;
use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::{util as util, TokenErrorType, TokenizeError};

pub fn is_eol(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_eol(token)
}

pub fn consume_eol(tokenizer: &mut Tokenizer) -> Result<(), TokenizeError>  {
    if !is_eol(tokenizer) {
        return tokenize_error!(TokenErrorType::UnexpectedToken, tokenizer);
    }

    tokenizer.next();
    tokenizer.found_new_line();

    return Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenErrorType};

    #[test]
    fn is_eol_input_eol() {
        let input = String::from_str("\n").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_eol(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_text_input_not_eol() {
        let input = String::from_str("Foobar").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_eol(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn consume_eol_input() {
        let input = String::from_str("\n").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        _ = super::consume_eol(&mut tokenizer).unwrap();

        assert_eq!(tokenizer.get_current_index(), input.len());
    }

    #[test]
    fn consume_invalid_input() {
        let input = String::from_str("🦀").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_eol(&mut tokenizer).unwrap_err();

        assert_eq!(token.error_type, TokenErrorType::UnexpectedToken);
    }

}
