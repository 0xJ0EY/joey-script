pub mod expression_statements;
pub mod block_statements;

#[macro_export]
macro_rules! ast_error {
    ($a: expr, $b: expr) => {
        Err(AstParseError { error_type: $a, index: $b.get_current_index() })
    };
}
