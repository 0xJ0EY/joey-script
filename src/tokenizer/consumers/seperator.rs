use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::{util as util, Token, TokenizeError, TokenType};

pub fn is_separator(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_separator(token)
}

pub fn consume_seperator(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError>  {
    let start = tokenizer.get_current_index();
    let token = tokenizer.consume().unwrap().to_string();
    let end = tokenizer.get_current_index();

    Ok(Token {
        token_type: TokenType::Seperator,
        value: token.clone(),
        raw_value: token,
        range: (start, end),
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenType};

    #[test]
    fn is_comma_input_a_seperator() {
        let input = String::from_str(",").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_separator(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_period_input_a_seperator() {
        let input = String::from_str(".").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_separator(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_whitespace_input_not_a_seperator() {
        let input = String::from_str(" ").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_separator(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn consume_comma_input() {
        let input = String::from_str(",").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_seperator(&mut tokenizer).unwrap();

        assert_eq!(tokenizer.get_current_index(), input.len());
        assert_eq!(token.token_type, TokenType::Seperator);
        assert_eq!(token.value, ",");
        assert_eq!(token.raw_value, ",");
    }

    #[test]
    fn consume_period_input() {
        let input = String::from_str(".").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_seperator(&mut tokenizer).unwrap();

        assert_eq!(tokenizer.get_current_index(), input.len());
        assert_eq!(token.token_type, TokenType::Seperator);
        assert_eq!(token.value, ".");
        assert_eq!(token.raw_value, ".");
    }

}
