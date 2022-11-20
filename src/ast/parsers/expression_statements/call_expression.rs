use crate::{ast::{parser::AstParser, nodes::{expression_statement::{ExpressionStatement, Expression, CallExpression}, Identifier}, AstParseError, AstErrorType, parsers::{function_declaration::parse_function_declaration, util::{parse_function_name}, expression_statements::parse_expression_statement}}, tokenizer::TokenType, ast_error};

use super::identifier_expression::parse_identifier_expression_statement;

pub fn is_call_expression_statement(parser: &AstParser) -> bool {
    match parser.token() {
        Some(token) => matches!(token.token_type, TokenType::Identifier),
        None => false,
    }
}

pub fn parse_call_expression_statement(parser: &mut AstParser) -> Result<ExpressionStatement, AstParseError> {
    let parse_identifier = |parser: &mut AstParser| -> Result<Identifier, AstParseError> {
        let expression = parse_identifier_expression_statement(parser)?;

        match expression.expression {
            Expression::Identifier(id) => Ok(id.identifier),
            _ => return ast_error!(AstErrorType::UnexpectedToken, parser)
        }
    };

    let parse_parameters = |parser: &mut AstParser| -> Result<Vec<Expression>, AstParseError> {
        
        parse_expression_statement(parser);
        
        todo!();
    };

    let identifier = parse_function_name(parser)?;
    let params = parse_parameters(parser)?;

    // let params = parse_parameters(parser);

    todo!()
}
