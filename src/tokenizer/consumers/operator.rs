use once_cell::sync::Lazy;

use crate::tokenize_error;
use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::{Token, TokenizeError, TokenType, FileLocation};

use super::is_word;

static OPERATORS: Lazy<Vec<&str>> = Lazy::new(|| {
    let mut operators = vec![
        // Assignment operators
        "=", "+=", "-=", "*=", "/=","%=", "**=",
        "<<=", ">>=",">>>=",
        "&=", "^=", "|=", "&&=", "||=", "??=",
        
        // Comparison operators
        "==", "!=", "===", "!==", ">", ">=", "<", "<=",

        // Arithmetic operators
        "%", "++", "--", "-", "+", "**",

        // Bitwise operators
        "&", "|", "^", "~", ">>", ">>>",

        // Logical operators
        "&&", "||", "!"
    ];
    
    operators.sort_by(|a, b| b.len().cmp(&a.len()));
    operators
});

pub fn is_operator(tokenizer: &Tokenizer, operator: &str) -> bool {
    is_word(tokenizer, operator)
}

pub fn find_operator(tokenizer: &Tokenizer) -> Result<&'static str, ()> {
    for operator in OPERATORS.iter() {
        if is_word(tokenizer, *operator) { return Ok(*&operator) }
    }

    return Err(())
}

pub fn consume_operator(tokenizer: &mut Tokenizer, operator: &str) -> Result<Token, TokenizeError> {
    if !is_operator(tokenizer, operator) { return tokenize_error!(crate::tokenizer::TokenErrorType::UnexpectedToken, tokenizer); }

    let start = tokenizer.get_current_index();
    let start_pos = tokenizer.get_current_file_loc();
    let mut raw_value = String::new();

    for _ in 0..operator.len() {
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
        token_type: TokenType::Operator,
        raw_value: value.clone(),
        value,
        range: (start, end),
        loc: FileLocation { start: start_pos, end: end_pos },
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenType, TokenErrorType};
    
    macro_rules! is_operator_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input = String::from_str($value).unwrap();
                let tokenizer = Tokenizer::new(&input);

                let result = super::is_operator(&tokenizer, $value);

                assert_eq!(result, true);
            }
        )*
        };
    }

    is_operator_tests! {
        is_plus_an_operator: "+",
        is_minus_an_operator: "-",
        is_is_an_operator: "=",
        is_gt_an_operator: ">",
        is_lt_an_operator: "<",
        is_exclamation_mark_an_operator: "!",
        is_divide_an_operator: "/",
        is_times_an_operator: "*",
        is_modulo_an_operator: "%",
        is_and_an_operator: "&",
        is_or_an_operator: "|",
        is_xor_an_operator: "^",
        is_not_an_operator: "~",
    }

    macro_rules! consume_operator_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input = String::from_str($value).unwrap();
                let mut tokenizer = Tokenizer::new(&input);
        
                let operator = super::consume_operator(&mut tokenizer, $value).unwrap();
        
                assert_eq!(operator.value, $value);
                assert_eq!(operator.raw_value, $value);
                assert_eq!(operator.token_type, TokenType::Operator);
            }
        )*
        };
    }

    consume_operator_tests! {
        consume_plus_an_operator: "+",
        consume_minus_an_operator: "-",
        consume_is_an_operator: "=",
        consume_gt_an_operator: ">",
        consume_lt_an_operator: "<",
        consume_exclamation_mark_an_operator: "!",
        consume_divide_an_operator: "/",
        consume_times_an_operator: "*",
        consume_modulo_an_operator: "%",
        consume_and_an_operator: "&",
        consume_or_an_operator: "|",
        consume_xor_an_operator: "^",
        consume_not_an_operator: "~",
    }

    #[test]
    fn operator_only_consumes_all_valid() {
        let input = String::from_str("-=-10").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let operator = super::consume_operator(&mut tokenizer, "-=").unwrap();

        assert_eq!(operator.value, "-=");
        assert_eq!(operator.raw_value, "-=");
        assert_eq!(operator.token_type, TokenType::Operator);
        assert_eq!(tokenizer.get_current_index(), 2);
    }

    #[test]
    fn consume_invalid_input() {
        let input = String::from_str("🦀").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        let token = super::consume_operator(&mut tokenizer, "++").unwrap_err();

        assert_eq!(token.error_type, TokenErrorType::UnexpectedToken);
    }

}
