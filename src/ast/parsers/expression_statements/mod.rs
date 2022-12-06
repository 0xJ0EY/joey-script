use crate::{ast::{parser::AstParser, AstParseError, nodes::expression_statement::ExpressionStatement, AstErrorType, SearchResult}, ast_error, tokenizer::{TokenType, Separator}};

use self::{literal_expression::{is_literal_expression_statement}, identifier_expression::is_identifier_expression_statement, call_expression::{is_call_expression_statement}, sequence_expression::is_sequence_expression_statement, binary_expression::is_binary_expression_statement};

pub mod identifier_expression;
pub mod literal_expression;
pub mod call_expression;
pub mod sequence_expression;
pub mod binary_expression;

type FindResult<T> = Result<Option<SearchResult<T>>, AstParseError>;

pub fn is_expression_statement(parser: &AstParser) -> bool {
    is_binary_expression_statement(parser) ||
    is_sequence_expression_statement(parser) ||
    is_literal_expression_statement(parser) ||
    is_identifier_expression_statement(parser) ||
    is_call_expression_statement(parser)
}

pub fn consume_result(parser: &mut AstParser, result: SearchResult<ExpressionStatement>) -> ExpressionStatement {
    let delta = result.ast_range.1 - result.ast_range.0;
    
    parser.consume_range(delta);

    result.value
}

pub fn parse_expression_statement(parser: &mut AstParser) -> Result<ExpressionStatement, AstParseError> {
    if let Some(result) = binary_expression::find(parser)? {
        return Ok(consume_result(parser, result));
    }

    if let Some(result) = call_expression::find(parser)? {
        return Ok(consume_result(parser, result));
    }

    if let Some(result) = literal_expression::find(parser)? {
        return Ok(consume_result(parser, result));
    }

    if let Some(result) = identifier_expression::find(parser)? {
        return Ok(consume_result(parser, result));
    }

    if let Some(result) = sequence_expression::find(parser)? {
        return Ok(consume_result(parser, result));
    }

    return ast_error!(AstErrorType::UnexpectedToken, parser);
}

fn expression_has_ended(parser: &AstParser, start_index: usize) -> bool {

    let index = start_index + 1;
    let end_marker = parser.token_at(index);

    match end_marker {
        Some(marker) => {
            if matches!(marker.token_type, TokenType::Separator(Separator::Terminator)) {
                return true;
            }

            if matches!(marker.token_type, TokenType::Separator(Separator::Comma)) {
                return true;
            }

            if index > 0 && parser.can_insert_automatic_semicolon(index) {
                return true;
            }

            return false
        },
        None => return true,
    }
}
