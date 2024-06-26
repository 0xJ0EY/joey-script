use crate::{tokenizer::{tokenizer::Tokenizer, TokenizeError, Token, TokenErrorType, Literal, TokenType, FileLocation}, tokenize_error};

use super::is_word;

fn is_true(tokenizer: &Tokenizer) -> bool {
    is_word(tokenizer, "true")
} 

fn is_false(tokenizer: &Tokenizer) -> bool {
    is_word(tokenizer, "false")
} 

pub fn is_boolean(tokenizer: &Tokenizer) -> bool {
    is_true(tokenizer) || is_false(tokenizer)
}

fn consume_value(tokenizer: &mut Tokenizer, length: usize) -> Result<Token, TokenizeError> {
    let start = tokenizer.get_current_index();
    let start_pos = tokenizer.get_current_file_loc();

    let mut raw_value = String::new();

    for _ in 0..length {
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
        token_type: TokenType::Literal(Literal::Boolean),
        value,
        raw_value,
        range: (start, end),
        loc: FileLocation { start: start_pos, end: end_pos }
    })
}

fn consume_true(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError> {
    consume_value(tokenizer, 4)
}

fn consume_false(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError> {
    consume_value(tokenizer, 5)
}

pub fn consume_boolean(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError> {

    if is_true(tokenizer) {
        return consume_true(tokenizer);
    }

    if is_false(tokenizer) {
        return consume_false(tokenizer);
    }

    return tokenize_error!(TokenErrorType::UnexpectedToken, tokenizer);
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenType, Literal, TokenErrorType};

    #[test]
    fn true_is_true_is_true() {
        let input = String::from_str("true").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_true(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn false_is_true_is_false() {
        let input = String::from_str("false").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_true(&tokenizer);

        assert_eq!(result, false);
    }
    
    #[test]
    fn false_is_false_is_true() {
        let input = String::from_str("false").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_false(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn true_is_false_is_false() {
        let input = String::from_str("true").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_false(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn false_is_boolean_is_true() {
        let input = String::from_str("false").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_boolean(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn true_is_boolean_is_true() {
        let input = String::from_str("true").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_boolean(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn true_consume_is_valid() {
        let input = String::from_str("true").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let result = super::consume_boolean(&mut tokenizer).unwrap();

        assert_eq!(result.token_type, TokenType::Literal(Literal::Boolean));
        assert_eq!(result.value, "true");
        assert_eq!(result.raw_value, "true");
        assert_eq!(tokenizer.get_current_index(), 4);
    }

    #[test]
    fn false_consume_is_valid() {
        let input = String::from_str("false").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let result = super::consume_boolean(&mut tokenizer).unwrap();

        assert_eq!(result.token_type, TokenType::Literal(Literal::Boolean));
        assert_eq!(result.value, "false");
        assert_eq!(result.raw_value, "false");
        assert_eq!(tokenizer.get_current_index(), 5);
    }

    #[test]
    fn consume_invalid_input() {
        let input = String::from_str("🦀").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_boolean(&mut tokenizer).unwrap_err();

        assert_eq!(token.error_type, TokenErrorType::UnexpectedToken);
    }

}


