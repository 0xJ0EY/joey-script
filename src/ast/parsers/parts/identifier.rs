use crate::{ast::{nodes::Identifier, AstParseError, parser::AstParser, AstErrorType}, tokenizer::TokenType, ast_error};

pub fn parse_identifier(parser: &AstParser) -> Result<Identifier, AstParseError> {
    match parser.token() {
        Some(token) => {
            if !matches!(token.token_type, TokenType::Identifier) {
                return ast_error!(AstErrorType::UnexpectedToken, parser);
            }

            Ok(Identifier::from(token))
        },
        None => return ast_error!(AstErrorType::UnexpectedToken, parser),
    }
}
