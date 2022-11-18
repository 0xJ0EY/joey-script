use crate::ast::{parser::AstParser, AstParseError, AstErrorType, nodes::expression_statement::ExpressionStatement};

use self::literal_expression::{is_literal_expression_statement, parse_literal_expression_statement};

pub mod literal_expression;

pub fn is_expression_statement(parser: &AstParser) -> bool {
    return is_literal_expression_statement(parser)
}

pub fn parse_expression_statement(parser: &mut AstParser) -> Result<ExpressionStatement, AstParseError> {
    if is_literal_expression_statement(parser) {
        return parse_literal_expression_statement(parser);
    }

    return Err(AstParseError { index: parser.get_current_index(), error_type: AstErrorType::UnexpectedToken });
}
