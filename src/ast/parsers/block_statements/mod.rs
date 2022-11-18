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

#[cfg(test)]
mod tests {
    use crate::{tokenizer, ast::{parser::AstParser, parsers::block_statements::{is_open_block_statement, is_closed_block_statement}, AstErrorType}};

    use super::parse_block_statement;

    #[test]
    fn open_bracket_is_open_block_statement() {
        let content = String::from("{");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_open_block_statement(&parser);

        assert_eq!(result, true);
    }

    #[test]
    fn closed_bracket_is_not_open_block_statement() {
        let content = String::from("}");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_open_block_statement(&parser);

        assert_eq!(result, false);
    }

    #[test]
    fn closed_bracket_is_closed_block_statement() {
        let content = String::from("}");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_closed_block_statement(&parser);

        assert_eq!(result, true);
    }

    #[test]
    fn open_bracket_is_not_closed_block_statement() {
        let content = String::from("{");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_closed_block_statement(&parser);

        assert_eq!(result, false);
    }

    #[test]
    fn block_with_literal_is_block_statement() {
        let content = String::from("{ 123 }");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let block = parse_block_statement(&mut parser).unwrap();

        assert_eq!(block.body.len(), 1);
    }

    #[test]
    fn block_with_subblock_with_literal_is_block_statement() {
        let content = String::from("{ { 123 } }");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let block = parse_block_statement(&mut parser).unwrap();

        assert_eq!(block.body.len(), 1);
    }

    #[test]
    fn block_without_closing_tag_throws_an_error() {
        let content = String::from("{ { 123 }");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let err = parse_block_statement(&mut parser).unwrap_err();

        assert_eq!(err.error_type, AstErrorType::UnexpectedEndOfInput);
    }

}