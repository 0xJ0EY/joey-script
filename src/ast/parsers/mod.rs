use self::expression_statements::literal_expression::{is_literal_expression_statement, parse_literal_expression_statement};

use super::{parser::AstParser, nodes::{expression_statement::ExpressionStatement, block_statement::BlockStatement}, AstParseError, AstErrorType};

pub mod expression_statements;

pub fn is_expression_statement(parser: &AstParser) -> bool {
    return is_literal_expression_statement(parser)
}

pub fn parse_expression_statement(parser: &mut AstParser) -> Result<ExpressionStatement, AstParseError> {
    if is_literal_expression_statement(parser) {
        return parse_literal_expression_statement(parser);
    }

    return Err(AstParseError { index: parser.get_current_index(), error_type: AstErrorType::UnexpectedToken });
}

pub fn is_block_statement(parser: &AstParser) -> bool {
    false
}

pub fn parse_block_statement(parser: &mut AstParser) -> Result<BlockStatement, AstParseError> {
    todo!()
}
