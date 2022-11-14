use std::vec;

use crate::{tokenizer::{tokenizer::Tokenizer, TokenizeError, Token, TokenErrorType, Literal, TokenType}, tokenize_error};

fn is_word(tokenizer: &Tokenizer, word: &Vec<char>) -> bool {

    for index in 0..word.len() {
        let char = tokenizer.peek_forward(index);
        
        if char.is_none() { return false }

        if char.unwrap().to_owned() != word[index] { return false }
    }

    true
}

fn is_true(tokenizer: &Tokenizer) -> bool {
    let word = vec!['t', 'r', 'u', 'e'];

    is_word(tokenizer, &word)
} 

fn is_false(tokenizer: &Tokenizer) -> bool {
    let word = vec!['f', 'a', 'l', 's', 'e'];

    is_word(tokenizer, &word)
} 

pub fn is_boolean(tokenizer: &Tokenizer) -> bool {
    is_true(tokenizer) || is_false(tokenizer)
}

fn consume_value(tokenizer: &mut Tokenizer, length: usize) -> Result<Token, TokenizeError> {
    let start = tokenizer.get_current_index();

    let mut raw_value = String::new();

    for _ in 0..length {
        raw_value.push(tokenizer
            .consume()
            .unwrap()
            .clone()
        );
    }

    let end  = tokenizer.get_current_index();
    let value = raw_value.clone();

    Ok(Token {
        token_type: TokenType::Literal(Literal::Boolean),
        value,
        raw_value,
        range: (start, end),
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

    use crate::tokenizer::{tokenizer::Tokenizer, TokenType, Literal};

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

}


