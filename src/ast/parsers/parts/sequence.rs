
use crate::{ast::{parser::AstParser, nodes::expression_statement::Expression, AstParseError}};

fn parse_sequence_token(parser: &AstParser, index: usize) -> Option<Expression> {
    todo!()
}

pub fn parse_sequence(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<Vec<Expression>, AstParseError> {
    todo!()
}
