use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::{util as util, Token, TokenType, TokenizeError};

pub fn is_identifier(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_identifier(token)
}

pub fn consume_identifier(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError> {
    let mut value = String::new();
    let start = tokenizer.get_current_index();

    let mut token = tokenizer.consume();

    while token.is_some() && util::is_identifier(&token.unwrap()) {
        value.push(token.unwrap().clone());

        token = tokenizer.consume();
    }
    
    tokenizer.walk_back();

    let end = tokenizer.get_current_index();
    let raw_value = value.clone();

    Ok(Token {
        token_type: TokenType::Identifier,
        raw_value,
        value,
        range: (start, end),
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenType};

    #[test]
    fn is_identifier_identifier() {
        let input = String::from_str("Foobar").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_identifier(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_number_an_identifier() {
        let input = String::from_str("1").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_identifier(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn consume_single_identifier_input() {
        let input = String::from_str("A").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_identifier(&mut tokenizer).unwrap();

        assert_eq!(token.value, "A");
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(tokenizer.get_current_index(), input.len());
    }

    #[test]
    fn consume_longer_identifier_input() {
        let input = String::from_str("Foobar").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_identifier(&mut tokenizer).unwrap();

        assert_eq!(token.value, "Foobar");
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(tokenizer.get_current_index(), input.len());
    }
}