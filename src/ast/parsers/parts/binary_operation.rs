use crate::{ast::{parser::AstParser, nodes::expression_statement::{BinaryExpression, Expression}, AstParseError}, ast_error, tokenizer::TokenType};

use super::{parse_non_sequence_expression, parse_binary_operation_expression};

fn parse_operator(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<String, AstParseError> {
    match parser.token_at(index) {
        Some(value) => {
            if matches!(value.token_type, TokenType::Operator) {
                *tokens_used += 1;

                return Ok(value.raw_value.clone())
            }

            ast_error!(crate::ast::AstErrorType::UnexpectedTokenStart, parser)
        },
        None => ast_error!(crate::ast::AstErrorType::UnexpectedTokenStart, parser),
    }
}

pub fn parse_binary_operation(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<BinaryExpression, AstParseError> {
    let mut tokens = 0;

    let lhs = parse_binary_operation_expression(parser, index + tokens, &mut tokens)?;

    let operator = parse_operator(parser, index + tokens, &mut tokens)?;

    let rhs = parse_binary_operation_expression(parser, index + tokens, &mut tokens)?;
    
    *tokens_used += tokens;

    Ok(BinaryExpression { operator, left: Box::new(lhs), right: Box::new(rhs) })
}



