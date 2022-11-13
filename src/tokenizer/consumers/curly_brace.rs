use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::{util as util, Token, TokenizeError, TokenType};

pub fn is_curly_brace(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_curly_brace(token)
}

pub fn consume_curly_brace(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError>  {
    let start = tokenizer.get_current_index();
    let token = tokenizer.consume().unwrap().to_string();
    let end = tokenizer.get_current_index();

    Ok(Token {
        token_type: TokenType::CurlyBrace,
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
    fn is_open_brace_input_a_curly_brace() {
        let input = String::from_str("{").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_curly_brace(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_closed_brace_input_a_curly_brace() {
        let input = String::from_str("}").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_curly_brace(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_whitespace_input_not_a_curly_brace() {
        let input = String::from_str(" ").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_curly_brace(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn consume_open_curly_brace_input() {
        let input = String::from_str("{").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_curly_brace(&mut tokenizer).unwrap();

        assert_eq!(tokenizer.get_current_index(), input.len());
        assert_eq!(token.token_type, TokenType::CurlyBrace);
        assert_eq!(token.value, "{");
        assert_eq!(token.raw_value, "{");
    }

    #[test]
    fn consume_closed_curly_brace_input() {
        let input = String::from_str("}").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_curly_brace(&mut tokenizer).unwrap();

        assert_eq!(tokenizer.get_current_index(), input.len());
        assert_eq!(token.token_type, TokenType::CurlyBrace);
        assert_eq!(token.value, "}");
        assert_eq!(token.raw_value, "}");
    }

}
