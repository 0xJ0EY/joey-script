use crate::{ast::{parser::AstParser, AstParseError, nodes::expression_statement::ExpressionStatement, AstErrorType, SearchResult}, ast_error};

use self::{literal_expression::{is_literal_expression_statement, parse_literal_expression_statement}, identifier_expression::{is_identifier_expression_statement, parse_identifier_expression_statement}, call_expression::{is_call_expression_statement, parse_call_expression_statement}, sequence_expression::is_sequence_expression_statement};

pub mod identifier_expression;
pub mod literal_expression;
pub mod call_expression;
pub mod sequence_expression;

type FindResult<T> = Result<SearchResult<T>, AstParseError>;

pub fn is_single_expression_statement(parser: &AstParser) -> bool {
    is_literal_expression_statement(parser) ||
    is_identifier_expression_statement(parser) ||
    is_call_expression_statement(parser)
}

pub fn is_expression_statement(parser: &AstParser) -> bool {
    is_sequence_expression_statement(parser) ||
    is_literal_expression_statement(parser) ||
    is_identifier_expression_statement(parser) ||
    is_call_expression_statement(parser)
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
