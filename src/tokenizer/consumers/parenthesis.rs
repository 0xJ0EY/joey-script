use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::{util as util, Token, TokenizeError, TokenType, Seperator};

pub fn is_parenthesis(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_parenthesis(token)
}

pub fn consume_parenthesis(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError>  {
    let start = tokenizer.get_current_index();
    let token = tokenizer.consume().unwrap().to_string();
    let end = tokenizer.get_current_index();

    Ok(Token {
        token_type: TokenType::Seperator(Seperator::Parenthesis),
        value: token.clone(),
        raw_value: token,
        range: (start, end),
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenType, Seperator};

    #[test]
    fn is_open_parenthesis_input_a_parenthesis() {
        let input = String::from_str("(").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_parenthesis(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_closed_parenthesis_input_a_parenthesis() {
        let input = String::from_str(")").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_parenthesis(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_whitespace_input_not_a_parenthesis() {
        let input = String::from_str(" ").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_parenthesis(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn consume_open_parenthesis_input() {
        let input = String::from_str("(").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_parenthesis(&mut tokenizer).unwrap();

        assert_eq!(tokenizer.get_current_index(), input.len());
        assert_eq!(token.token_type, TokenType::Seperator(Seperator::Parenthesis));
        assert_eq!(token.value, "(");
        assert_eq!(token.raw_value, "(");
    }

    #[test]
    fn consume_closed_parenthesis_input() {
        let input = String::from_str(")").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_parenthesis(&mut tokenizer).unwrap();

        assert_eq!(tokenizer.get_current_index(), input.len());
        assert_eq!(token.token_type, TokenType::Seperator(Seperator::Parenthesis));
        assert_eq!(token.value, ")");
        assert_eq!(token.raw_value, ")");
    }

}
