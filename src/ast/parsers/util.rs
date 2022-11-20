use crate::{ast::{parser::AstParser, nodes::{Identifier, expression_statement::Expression}, AstParseError, AstErrorType}, tokenizer::{TokenType, Separator}, ast_error};

use super::expression_statements::identifier_expression::parse_identifier_expression_statement;


// Parsing util functions
pub fn is_open_param_bracket(parser: &mut AstParser) -> bool {
    match parser.token() {
        Some(token) => {
            let is_type = matches!(token.token_type, TokenType::Separator(Separator::Parenthesis));
            let is_value = token.value == "(";

            is_type && is_value
        },
        None => false,
    }
}

pub fn is_param_separator(parser: &mut AstParser) -> bool {
    match parser.token() {
        Some(token) => matches!(token.token_type, TokenType::Separator(Separator::Comma)),
        None => false,
    }
}

pub fn is_closed_param_bracket(parser: &mut AstParser) -> bool {
    match parser.token() {
        Some(token) => {
            let is_type = matches!(token.token_type, TokenType::Separator(Separator::Parenthesis));
            let is_value = token.value == ")";

            is_type && is_value
        },
        None => false,
    }
}

pub fn parse_function_name(parser: &mut AstParser) -> Result<Identifier, AstParseError> {
    let expression = parse_identifier_expression_statement(parser)?;

    match expression.expression {
        Expression::Identifier(id) => Ok(id.identifier),
        _ => return ast_error!(AstErrorType::UnexpectedToken, parser)
    }
}

pub fn is_semicolon_terminator(parser: &AstParser) -> bool {
    match parser.token() {
        Some(token) => {
            matches!(token.token_type, TokenType::Separator(Separator::Terminator))
        },
        None => false,
    }
}
