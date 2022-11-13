use crate::tokenizer::tokenizer::Tokenizer;
use crate::tokenizer::{util as util, Token, TokenizeError, TokenType};

// Note: An operator may only be 1 character long, of what kind the operator exists should be later handeld in the AST parsing

pub fn is_operator(tokenizer: &Tokenizer) -> bool {
    let token = tokenizer.token().unwrap();

    util::is_operator(token)
}

pub fn consume_operator(tokenizer: &mut Tokenizer) -> Result<Token, TokenizeError> {
    let mut value = String::new();
    let start = tokenizer.get_current_index();

    let token = tokenizer.token();

    if token.is_some() && is_operator(&tokenizer) {
        value.push(token.unwrap().clone());

        tokenizer.next();
    }

    let end = tokenizer.get_current_index();

    Ok(Token {
        token_type: TokenType::Operator,
        raw_value: value.clone(),
        value,
        range: (start, end)
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::{tokenizer::Tokenizer, TokenType};
    
    macro_rules! is_operator_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let input = String::from_str($value).unwrap();
                let tokenizer = Tokenizer::new(&input);

                let result = super::is_operator(&tokenizer);

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
        
                let operator = super::consume_operator(&mut tokenizer).unwrap();
        
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

}
