use crate::{ast::{parser::AstParser, nodes::block_statement::BlockStatement, AstParseError, AstErrorType}, tokenizer::{Separator, TokenType}, ast_error};

pub fn is_open_block_statement(parser: &AstParser) -> bool {
    let token = parser.token().unwrap();

    let is_type = matches!(token.token_type, TokenType::Separator(Separator::CurlyBrace));
    let is_value = token.value == "{";

    is_type && is_value
}

pub fn is_closed_block_statement(parser: &AstParser) -> bool {
    let token = parser.token().unwrap();

    let is_type = matches!(token.token_type, TokenType::Separator(Separator::CurlyBrace));
    let is_value = token.value == "}";

    is_type && is_value
}

pub fn parse_block_statement(parser: &mut AstParser) -> Result<BlockStatement, AstParseError> {
    if !is_open_block_statement(&parser) {
        return ast_error!(AstErrorType::UnexpectedToken, parser);
    }

    let start = parser.token().unwrap().range.0;

    // Skip the current opening block
    parser.next();

    // Fetch all expressions within the block
    let body = parser.parse_block()?;

    let end = parser.peek_back().unwrap().range.1;

    Ok(BlockStatement {
        body,
        range: (start, end)
    })
}
