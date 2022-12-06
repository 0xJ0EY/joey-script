use crate::{ast::{parser::AstParser, nodes::expression_statement::{BinaryExpression, Expression}, AstParseError, AstErrorType}, ast_error, tokenizer::TokenType};

use super::{function_call::parse_function_call, literal::parse_literal, identifier::parse_identifier};

fn parse_binary_operation_expression(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<Expression, AstParseError> {
    // TODO: Add the other expressions when implemented
    if let Ok(result) = parse_function_call(parser, index, tokens_used) {
        return Ok(Expression::CallExpression(result));
    }

    if let Ok(result) = parse_literal(parser, index, tokens_used) {
        return Ok(Expression::Literal(result));
    }

    if let Ok(result) = parse_identifier(parser, index, tokens_used) {
        return Ok(Expression::Identifier(result));
    }

    ast_error!(AstErrorType::UnexpectedToken, parser)
}

fn parse_operator(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<String, AstParseError> {
    match parser.token_at(index) {
        Some(value) => {
            if matches!(value.token_type, TokenType::Operator) {
                *tokens_used += 1;

                return Ok(value.raw_value.clone())
            }

            ast_error!(crate::ast::AstErrorType::UnexpectedTokenStart, parser)
        },
        None => ast_error!(crate::ast::AstErrorType::UnexpectedTokenStart, parser),
    }
}

pub fn parse_binary_operation(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<BinaryExpression, AstParseError> {
    let mut tokens = 0;
    let mut binary_expression;

    let left = Box::new(parse_binary_operation_expression(parser, index + tokens, &mut tokens)?);
    let operator = parse_operator(parser, index + tokens, &mut tokens)?;
    let right = Box::new(parse_binary_operation_expression(parser, index + tokens, &mut tokens)?);

    binary_expression = BinaryExpression { operator, left, right };

    while let Ok(operator) = parse_operator(parser, index + tokens, &mut tokens) {
        let left    = Box::new(Expression::BinaryExpression(binary_expression));
        let right   = Box::new(parse_binary_operation_expression(parser, index + tokens, &mut tokens)?);

        binary_expression = BinaryExpression { operator, left, right };
    }

    *tokens_used += tokens;

    Ok(binary_expression)
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer, ast::{parser::AstParser, nodes::expression_statement::Expression}, cast_expression};

    #[test]
    fn binary_expression_is_binary_operation() {
        let content = String::from("321 + 123");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);
        let mut tokens_used = 0;

        let result = super::parse_binary_operation(&parser, 0, &mut tokens_used).unwrap();

        let expected_left = cast_expression!(result.left.as_ref(), Expression::Literal).unwrap();
        let expected_right = cast_expression!(result.right.as_ref(), Expression::Literal).unwrap();

        assert_eq!(result.operator, "+");
        assert_eq!(expected_left.value.raw, "321");
        assert_eq!(expected_right.value.raw, "123");
        assert_eq!(tokens_used, 3);
    }

    #[test]
    fn multiple_binary_expression_is_binary_operation() {
        let content = String::from("112233 - 321 + 123");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);
        let mut tokens_used = 0;

        let result = super::parse_binary_operation(&parser, 0, &mut tokens_used).unwrap();

        let expected_left = cast_expression!(result.left.as_ref(), Expression::BinaryExpression).unwrap();

        let expected_left_left = cast_expression!(expected_left.left.as_ref(), Expression::Literal).unwrap();
        let expected_left_right = cast_expression!(expected_left.right.as_ref(), Expression::Literal).unwrap();

        let expected_right = cast_expression!(result.right.as_ref(), Expression::Literal).unwrap();

        assert_eq!(expected_left_left.value.raw, "112233");
        assert_eq!(expected_left.operator, "-");
        assert_eq!(expected_left_right.value.raw, "321");
        assert_eq!(result.operator, "+");
        assert_eq!(expected_right.value.raw, "123");

        assert_eq!(tokens_used, 5);
    }

}
