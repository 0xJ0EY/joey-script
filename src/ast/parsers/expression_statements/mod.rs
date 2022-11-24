use crate::{ast::{parser::AstParser, AstParseError, nodes::expression_statement::ExpressionStatement, AstErrorType, SearchResult}, ast_error};

use self::{literal_expression::{is_literal_expression_statement}, identifier_expression::is_identifier_expression_statement, call_expression::{is_call_expression_statement, parse_call_expression_statement}, sequence_expression::is_sequence_expression_statement};

pub mod identifier_expression;
pub mod literal_expression;
pub mod call_expression;
pub mod sequence_expression;

type FindResult<T> = Result<Option<SearchResult<T>>, AstParseError>;

pub fn is_expression_statement(parser: &AstParser) -> bool {
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
    
    if is_call_expression_statement(parser) {
        return parse_call_expression_statement(parser);
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
