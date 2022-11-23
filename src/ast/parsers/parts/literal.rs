use crate::{ast::{parser::AstParser, nodes::Literal, AstParseError, AstErrorType}, ast_error, tokenizer::TokenType};

pub fn parse_literal(parser: &AstParser) -> Result<Literal, AstParseError> {
    match parser.token() {
        Some(token) => {
            if !matches!(token.token_type, TokenType::Literal(_)) {
                return ast_error!(AstErrorType::UnexpectedToken, parser);
            }

            Ok(Literal::from(token))
        },
        None => return ast_error!(AstErrorType::UnexpectedToken, parser),
    }
}

