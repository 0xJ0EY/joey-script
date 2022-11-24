use crate::{ast::{parser::AstParser, nodes::Literal, AstParseError, AstErrorType}, ast_error, tokenizer::TokenType};

pub fn parse_literal(parser: &AstParser, index: usize, used_tokens: &mut usize) -> Result<Literal, AstParseError> {
    match parser.token_at(index) {
        Some(token) => {
            if !matches!(token.token_type, TokenType::Literal(_)) {
                return ast_error!(AstErrorType::UnexpectedToken, parser);
            }

            *used_tokens = 1;

            Ok(Literal::from(token))
        },
        None => return ast_error!(AstErrorType::UnexpectedToken, parser),
    }
}

