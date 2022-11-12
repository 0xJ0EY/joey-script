pub mod whitespace;
pub mod number;
pub mod identifier;
pub mod string;
pub mod comments;

#[macro_export]
macro_rules! tokenize_error {
    ($a: expr, $b: expr) => {
        Err(TokenizeError { error_type: $a, index: $b.get_current_index() })
    };
}