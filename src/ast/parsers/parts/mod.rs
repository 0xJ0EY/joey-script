use crate::{ast::{parser::AstParser, AstParseError, nodes::expression_statement::Expression, AstErrorType}, ast_error};

use self::{identifier::parse_identifier, literal::parse_literal, function_call::parse_function_call, binary_operation::parse_binary_operation};

pub mod literal;
pub mod identifier;
pub mod sequence;
pub mod function_call;
pub mod binary_operation;

fn parse_non_sequence_expression(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<Expression, AstParseError> {
    // TODO: Add the other expressions when implemented
    // Out of my head those are ArrayExpression, ObjectExpression, MemberExpression, UpdateExpression
    if let Ok(result) = parse_binary_operation(parser, index, tokens_used) {
        return Ok(Expression::BinaryExpression(result));
    }

    if let Ok(result) = parse_function_call(parser, index, tokens_used) {
        return Ok(Expression::CallExpression(result));
    }

    if let Ok(result) = parse_literal(parser, index, tokens_used) {
        return Ok(Expression::Literal(result));
    }

    if let Ok(result) = parse_identifier(parser, index, tokens_used) {
        return Ok(Expression::Identifier(result));
    }

    ast_error!(AstErrorType::UnexpectedToken, parser)
}
