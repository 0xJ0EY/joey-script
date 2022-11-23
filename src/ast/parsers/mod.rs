use super::{AstParseError, parser::AstParser, AstErrorType};

pub mod expression_statements;
pub mod block_statements;
pub mod function_declaration;
pub mod util;


#[macro_export]
macro_rules! handle_allowed_find_error {
    ($a: expr) => {
        match $a {
            Ok(token) => token,
            Err(_) => return Ok(None),
        }
    };
}

#[macro_export]
macro_rules! ast_error {
    ($a: expr, $b: expr) => {
        Err(AstParseError { error_type: $a, index: $b.get_current_index() })
    };
}

fn get_start_position(parser: &AstParser) -> Result<usize, AstParseError> {
    match parser.token() {
        Some(token) => Ok(token.range.0),
        None => ast_error!(AstErrorType::UnexpectedToken, parser),
    } 
}

fn get_end_position(parser: &AstParser) -> Result<usize, AstParseError> {
    match parser.token() {
        Some(token) => Ok(token.range.1),
        None => ast_error!(AstErrorType::UnexpectedToken, parser),
    } 
}

fn get_end_position_of_previous_token(parser: &AstParser) -> Result<usize, AstParseError> {
    match parser.peek_back() {
        Some(token) => Ok(token.range.1),
        None => ast_error!(AstErrorType::UnexpectedToken, parser),
    } 
}
