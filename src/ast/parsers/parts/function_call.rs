use crate::ast::{AstParseError, parser::AstParser, nodes::expression_statement::CallExpression};

pub fn parse_function_call(parser: &AstParser) -> Result<CallExpression, AstParseError> {
    todo!()
}
