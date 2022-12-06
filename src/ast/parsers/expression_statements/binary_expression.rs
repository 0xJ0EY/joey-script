use crate::{ast::{nodes::expression_statement::{Expression, ExpressionStatement}, AstParseError, parsers::parts::binary_operation::parse_binary_operation, AstErrorType, parser::AstParser, SearchResult}, ast_error};

use super::{FindResult, expression_has_ended};

pub fn is_binary_expression_statement(parser: &AstParser) -> bool {
    if let Ok(response) = find(parser) {
        return response.is_some()
    }

    false
}

pub fn find(parser: &AstParser) -> FindResult<ExpressionStatement> {
    let start_index = parser.get_current_index();
    let mut used_tokens = 0;

    let binary_expression = match parse_binary_operation(parser, start_index, &mut used_tokens) {
        Ok(exp) => exp,
        Err(err) => match err.error_type {
            AstErrorType::UnexpectedTokenStart => return Ok(None),
            _ => return Err(err)
        },
    };

    if !expression_has_ended(parser, start_index + used_tokens) {
        return ast_error!(AstErrorType::UnexpectedToken, parser);
    }

    // Consume end token
    used_tokens += 1;

    // TODO: Implement correct function distance
    let literal_start   = 0;
    let literal_end     = 0;

    let ast_start = start_index;
    let ast_end = ast_start + used_tokens;

    let expression_statement = ExpressionStatement {
        expression: Expression::BinaryExpression(binary_expression),
        range: (literal_start, literal_end)
    };

    Ok(Some(SearchResult::<ExpressionStatement> {
        value: expression_statement,
        ast_range: (ast_start, ast_end),
    }))
}

#[cfg(test)]
mod tests {
    use crate::{ast::{parser::AstParser, nodes::expression_statement::Expression}, tokenizer, cast_expression_statement, cast_expression};

    #[test]
    fn binary_operation_is_binary_expression_statement() {
        let content = String::from("123 + 123");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = super::is_binary_expression_statement(&parser);

        assert_eq!(result, true);
    }

    #[test]
    fn literal_is_not_a_binary_expression_statement() {
        let content = String::from("123123");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = super::is_binary_expression_statement(&parser);

        assert_eq!(result, false);
    }

    #[test]
    fn malformed_binary_operation_is_not_a_binary_expression_statement() {
        let content = String::from("123 + 123 -");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = super::is_binary_expression_statement(&parser);

        assert_eq!(result, false);
    }

    #[test]
    fn binary_operation_is_parsable_binary_operation() {
        let content = String::from("123 + 123");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = super::find(&mut parser).unwrap().unwrap().value;

        let expected = cast_expression_statement!(result, Expression::BinaryExpression).unwrap();

        let expected_left = cast_expression!(expected.left.as_ref(), Expression::Literal).unwrap();
        let expected_right = cast_expression!(expected.right.as_ref(), Expression::Literal).unwrap();

        assert_eq!(expected_left.value.raw, "123");
        assert_eq!(expected_right.value.raw, "123");
        assert_eq!(expected.operator, "+")
    }

    #[test]
    fn multiple_binary_operation_is_parsable_binary_operation() {
        let content = String::from("123 + 123 - 123");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = super::find(&mut parser).unwrap().unwrap().value;

        let expected = cast_expression_statement!(result, Expression::BinaryExpression).unwrap();

        let expected_left = cast_expression!(expected.left.as_ref(), Expression::BinaryExpression).unwrap();
        let expected_left_left = cast_expression!(expected_left.left.as_ref(), Expression::Literal).unwrap();
        let expected_left_right = cast_expression!(expected_left.left.as_ref(), Expression::Literal).unwrap();

        let expected_right = cast_expression!(expected.right.as_ref(), Expression::Literal).unwrap();

        assert_eq!(expected_left_left.value.raw, "123");
        assert_eq!(expected_left.operator, "+");
        assert_eq!(expected_left_right.value.raw, "123");
        assert_eq!(expected_right.value.raw, "123");
        assert_eq!(expected.operator, "-")
    }

}

