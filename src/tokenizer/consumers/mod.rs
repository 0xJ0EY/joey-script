pub mod whitespace;
pub mod number;
pub mod identifier;
pub mod string;
pub mod comments;
pub mod operator;
pub mod terminator;
pub mod seperator;
pub mod parenthesis;
pub mod curly_brace;

#[macro_export]
macro_rules! tokenize_error {
    ($a: expr, $b: expr) => {
        Err(TokenizeError { error_type: $a, index: $b.get_current_index() })
    };
}