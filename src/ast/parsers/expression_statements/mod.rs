use crate::{ast::{parser::AstParser, AstParseError, nodes::expression_statement::ExpressionStatement, AstErrorType}, ast_error, tokenizer::{TokenType, Separator}};

use self::{literal_expression::{is_literal_expression_statement, parse_literal_expression_statement}, identifier_expression::{is_identifier_expression_statement, parse_identifier_expression_statement}, call_expression::{is_call_expression_statement, parse_call_expression_statement}};

use super::block_statements::is_closed_block_statement;

pub mod identifier_expression;
pub mod literal_expression;
pub mod call_expression;

pub fn is_expression_statement(parser: &AstParser) -> bool {
    is_literal_expression_statement(parser) || is_identifier_expression_statement(parser)
}

pub fn parse_expression_statement(parser: &mut AstParser) -> Result<ExpressionStatement, AstParseError> {
    if is_call_expression_statement(parser) {
        return parse_call_expression_statement(parser);
    }

    if is_literal_expression_statement(parser) {
        return parse_literal_expression_statement(parser);
    }

    if is_identifier_expression_statement(parser) {
        return parse_identifier_expression_statement(parser);
    }

    return ast_error!(AstErrorType::UnexpectedToken, parser);
}

pub fn check_if_expression_has_ended(parser: &mut AstParser) -> bool {
    let end_marker = parser.token();

    match end_marker {
        Some(marker) => {
            if matches!(marker.token_type, TokenType::Separator(Separator::Terminator)) {
                return true;
            }

            if matches!(marker.token_type, TokenType::Separator(Separator::Comma)) {
                return true;
            }

            if is_closed_block_statement(parser) {
                return true;
            }

            let index = parser.get_current_index() ;
            if index > 0 && parser.can_insert_automatic_semicolon(index) {
                return true;
            }

            return false
        },
        None => return true,
    }
}
