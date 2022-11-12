use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::{util as util, Token, TokenType, TokenizeError};

pub fn is_number(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_number(token)
}

pub fn consume_number(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError> {
    let mut value = String::new();
    let start = tokenizer.get_current_index();


    let mut token = tokenizer.consume();

    while token.is_some() && util::is_number(&token.unwrap()) {
        value.push(token.unwrap().clone());


        token = tokenizer.consume();
    }
    
    tokenizer.walk_back();

    let end = tokenizer.get_current_index();
    let raw_value = value.clone();

    Ok(Token {
        token_type: TokenType::Number,
        raw_value,
        value,
        range: (start, end),
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::tokenizer::Tokenizer;

    #[test]
    fn is_number_a_number() {
        let input = String::from_str("1").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_number(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_text_input_not_a_number() {
        let input = String::from_str("Foobar").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_number(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn consume_single_number_input() {
        let input = String::from_str("1").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_number(&mut tokenizer).unwrap();

        assert_eq!(token.value, "1");
        assert_eq!(tokenizer.get_current_index(), input.len());
    }

    #[test]
    fn consume_double_digit_number_input() {
        let input = String::from_str("123").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_number(&mut tokenizer).unwrap();

        assert_eq!(token.value, "123");
        assert_eq!(tokenizer.get_current_index(), input.len());
    }

}