use crate::ast::{parser::AstParser, AstParseError, nodes::expression_statement::ExpressionStatement, AstErrorType};

use self::{literal_expression::{is_literal_expression_statement, parse_literal_expression_statement}, identifier_expression::{is_identifier_expression_statement, parse_identifier_expression_statement}};

pub mod identifier_expression;
pub mod literal_expression;

pub fn is_expression_statement(parser: &AstParser) -> bool {
    is_literal_expression_statement(parser)|| is_identifier_expression_statement(parser)
}

pub fn parse_expression_statement(parser: &mut AstParser) -> Result<ExpressionStatement, AstParseError> {
    if is_literal_expression_statement(parser) {
        return parse_literal_expression_statement(parser);
    }

    if is_identifier_expression_statement(parser) {
        return parse_identifier_expression_statement(parser);
    }

    return Err(AstParseError { index: parser.get_current_index(), error_type: AstErrorType::UnexpectedToken });
}
