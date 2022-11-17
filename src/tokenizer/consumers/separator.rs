use crate::tokenize_error;
use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::{util as util, Token, TokenizeError, TokenType, Separator};

pub fn is_period(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_period(token)
}

pub fn is_comma(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_comma(token)
}

pub fn consume_period(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError>  {
    if !is_period(tokenizer) {
        return tokenize_error!(crate::tokenizer::TokenErrorType::UnexpectedToken, tokenizer);
    }

    let start = tokenizer.get_current_index();
    let token = tokenizer.consume().unwrap().to_string();
    let end = tokenizer.get_current_index();

    Ok(Token {
        token_type: TokenType::Separator(Separator::Period),
        value: token.clone(),
        raw_value: token,
        range: (start, end),
    })
}

pub fn consume_comma(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError>  {
    if !is_comma(tokenizer) {
        return tokenize_error!(crate::tokenizer::TokenErrorType::UnexpectedToken, tokenizer);
    }

    let start = tokenizer.get_current_index();
    let token = tokenizer.consume().unwrap().to_string();
    let end = tokenizer.get_current_index();

    Ok(Token {
        token_type: TokenType::Separator(Separator::Comma),
        value: token.clone(),
        raw_value: token,
        range: (start, end),
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenType, Separator, TokenErrorType};

    #[test]
    fn is_comma_input_a_separator() {
        let input = String::from_str(",").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_comma(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_period_input_a_separator() {
        let input = String::from_str(".").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_period(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_whitespace_input_not_a_separator() {
        let input = String::from_str(" ").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_comma(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn consume_comma_input() {
        let input = String::from_str(",").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_comma(&mut tokenizer).unwrap();

        assert_eq!(tokenizer.get_current_index(), input.len());
        assert_eq!(token.token_type, TokenType::Separator(Separator::Comma));
        assert_eq!(token.value, ",");
        assert_eq!(token.raw_value, ",");
    }

    #[test]
    fn consume_period_input() {
        let input = String::from_str(".").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_period(&mut tokenizer).unwrap();

        assert_eq!(tokenizer.get_current_index(), input.len());
        assert_eq!(token.token_type, TokenType::Separator(Separator::Period));
        assert_eq!(token.value, ".");
        assert_eq!(token.raw_value, ".");
    }

    #[test]
    fn consume_invalid_input_period() {
        let input = String::from_str("🦀").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_period(&mut tokenizer).unwrap_err();

        assert_eq!(token.error_type, TokenErrorType::UnexpectedToken);
    }

    #[test]
    fn consume_invalid_input_comma() {
        let input = String::from_str("🦀").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_comma(&mut tokenizer).unwrap_err();

        assert_eq!(token.error_type, TokenErrorType::UnexpectedToken);
    }

}
