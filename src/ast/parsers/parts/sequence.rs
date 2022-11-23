
use crate::{ast::{parser::AstParser, nodes::Literal, AstParseError, AstErrorType}, ast_error, tokenizer::TokenType};

pub fn parse_sequence(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<Literal, AstParseError> {
    todo!()
}
