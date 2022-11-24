use crate::{ast::{nodes::{Identifier, expression_statement::IdentifierExpression}, AstParseError, parser::AstParser, AstErrorType}, tokenizer::TokenType, ast_error};

pub fn parse_identifier(parser: &AstParser, index: usize, used_tokens: &mut usize) -> Result<IdentifierExpression, AstParseError> {
    match parser.token_at(index) {
        Some(token) => {
            if !matches!(token.token_type, TokenType::Identifier) {
                return ast_error!(AstErrorType::UnexpectedToken, parser);
            }

            *used_tokens += 1;

            let identifier = Identifier::from(token);

            Ok(IdentifierExpression { identifier })
        },
        None => return ast_error!(AstErrorType::UnexpectedToken, parser),
    }
}
