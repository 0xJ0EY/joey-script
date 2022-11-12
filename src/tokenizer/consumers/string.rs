use crate::tokenizer::util::{is_eol, is_escape_char};
use crate::{tokenize_error};
use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::{util as util, Token, TokenizeError, TokenErrorType, TokenType};

pub fn is_string(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_string_delimiter(token)
}

fn is_end_string(tokenizer: &Tokenizer, delimiter: char) -> bool {
    if is_escaped(tokenizer, tokenizer.get_current_index() - 1) { return false; }

    // because consume() ups the index by one, we need to peek back
    let token = tokenizer.peek_back().unwrap();

    token.clone() == delimiter
}

fn is_escaped(tokenizer: &Tokenizer, index: usize) -> bool {
    tokenizer.token_at(index - 1)
        .unwrap_or(&' ')
        .clone() == '\\'
}

macro_rules! unwrap_token {
    ($a:expr) => {
        match $a.consume() {
            Some(val) => Some(val.to_owned()),
            None => None,
        }
    };
}

pub fn consume_string(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError> {
    // We clone the delimiter here, otherwise the tokenizer will have a mutable reference
    let delimiter = tokenizer.consume().unwrap().clone();

    if !util::is_string_delimiter(&delimiter) {
        return tokenize_error!(TokenErrorType::UnexpectedToken, tokenizer);
    }
    
    let mut raw_value = String::new();
    raw_value.push(delimiter);

    let mut value = String::new();
    let start = tokenizer.get_current_index();

    let mut index = tokenizer.get_current_index();
    let mut token = unwrap_token!(tokenizer);
    
    while token.is_some() && !is_end_string(tokenizer, delimiter) {
        let val = token.unwrap().clone();

        if is_eol(&val) {
            return tokenize_error!(TokenErrorType::UnterminatedStringLiteral, tokenizer);
        }

        raw_value.push(val);

        if !is_escape_char(&val) || is_escaped(tokenizer, index) {
            value.push(val);
        }

        index = tokenizer.get_current_index();
        token = unwrap_token!(tokenizer);
    }

    // the last token should be the same as the first delimiter
    if token.is_some() && (token.unwrap() == delimiter) {
        let val = token.unwrap().clone();
        raw_value.push(val);
        
    } else {
        return tokenize_error!(TokenErrorType::UnterminatedStringLiteral, tokenizer);
    }

    let end = tokenizer.get_current_index();

    Ok(Token {
        token_type: TokenType::String,
        value,
        raw_value,
        range: (start, end)
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenType, TokenErrorType};

    #[test]
    fn is_string_a_string() {
        let input = String::from_str("\"Foobar\"").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_string(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn number_is_not_a_string() {
        let input = String::from_str("1").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_string(&tokenizer);

        assert_eq!(result, false);
    }

    #[test]
    fn consume_string_input() {
        let input = String::from_str("\"Foobar\"").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_string(&mut tokenizer).unwrap(   );

        assert_eq!(token.value, "Foobar");
        assert_eq!(token.raw_value, "\"Foobar\"");
        assert_eq!(token.token_type, TokenType::String);
        assert_eq!(tokenizer.get_current_index(), input.len());
    }

    #[test]
    fn consume_string_input_with_escaped_quote() {
        let input = String::from_str("\"Foo\\\"bar\"").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_string(&mut tokenizer).unwrap(   );

        assert_eq!(token.value, "Foo\"bar");
        assert_eq!(token.raw_value, "\"Foo\\\"bar\"");
        assert_eq!(token.token_type, TokenType::String);
        assert_eq!(tokenizer.get_current_index(), input.len());
    }

    #[test]
    fn consume_string_input_with_escaped_char() {
        let input = String::from_str("\"Foo\\abar\"").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_string(&mut tokenizer).unwrap(   );

        assert_eq!(token.value, "Fooabar");
        assert_eq!(token.raw_value, "\"Foo\\abar\"");
        assert_eq!(token.token_type, TokenType::String);
        assert_eq!(tokenizer.get_current_index(), input.len());
    }

    #[test]
    fn consume_string_input_with_escaped_slash() {
        let input = String::from_str("\"Foo\\\\bar\"").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_string(&mut tokenizer).unwrap(   );

        assert_eq!(token.value, "Foo\\bar");
        assert_eq!(token.raw_value, "\"Foo\\\\bar\"");
        assert_eq!(token.token_type, TokenType::String);
        assert_eq!(tokenizer.get_current_index(), input.len());
    }


    #[test]
    fn give_error_on_line_seperated_string() {
        let input = String::from_str("\"Foo\nbar\"").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_string(&mut tokenizer).unwrap_err();

        assert_eq!(token.error_type, TokenErrorType::UnterminatedStringLiteral);
    }

    #[test]
    fn give_error_on_unterminated_string() {
        let input = String::from_str("\"Foo").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_string(&mut tokenizer).unwrap_err();

        assert_eq!(token.error_type, TokenErrorType::UnterminatedStringLiteral);
    }

    #[test]
    fn give_error_on_incorrect_terminated_string() {
        let input = String::from_str("\"Foo'").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_string(&mut tokenizer).unwrap_err();

        assert_eq!(token.error_type, TokenErrorType::UnterminatedStringLiteral);
    }
    
}
