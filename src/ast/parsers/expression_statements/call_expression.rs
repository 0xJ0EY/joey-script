use crate::{ast::{parser::AstParser, nodes::{expression_statement::{ExpressionStatement, Expression}}, AstParseError, AstErrorType, parsers::{parts::function_call::parse_function_call}, SearchResult}, ast_error, handle_allowed_find_error};

use super::{FindResult, expression_has_ended};

pub fn is_call_expression_statement(parser: &AstParser) -> bool {
    if let Ok(response) = find(parser) {
        return response.is_some()
    }

    false
}

pub fn find(parser: &AstParser) -> FindResult<ExpressionStatement> {
    let start_index = parser.get_current_index();
    let mut used_tokens = 0;

    let call_expression = handle_allowed_find_error!(parse_function_call(parser, start_index, &mut used_tokens));
    let call = &call_expression.callee;

    if !expression_has_ended(parser, start_index + used_tokens) {
        return ast_error!(AstErrorType::UnexpectedToken, parser);
    }

    // Consume end token
    used_tokens += 1;

    // TODO: Implement correct function distance
    let literal_start = call.range.0;
    let literal_end =  call.range.1;

    let ast_start = start_index;
    let ast_end = ast_start + used_tokens;

    let expression_statement = ExpressionStatement {
        expression: Expression::CallExpression(call_expression),
        range: (literal_start, literal_end)
    };

    Ok(Some(SearchResult::<ExpressionStatement> {
        value: expression_statement,
        ast_range: (ast_start, ast_end),
    }))
}

#[cfg(test)]
mod tests {

    use crate::{ast::{parsers::expression_statements::call_expression::is_call_expression_statement, parser::AstParser, nodes::expression_statement::Expression, AstErrorType}, tokenizer, cast_expression_statement};

    use super::find;

    #[test]
    fn function_call_without_params_is_call_expression_statement() {
        let content = String::from("call()");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_call_expression_statement(&parser);

        assert_eq!(result, true);
    }

    #[test]
    fn function_call_with_params_is_call_expression_statement() {
        let content = String::from("call('123', 123, call())");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_call_expression_statement(&parser);

        assert_eq!(result, true);
    }

    #[test]
    fn identifier_is_not_a_call_expression_statement() {
        let content = String::from("foobar");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_call_expression_statement(&parser);

        assert_eq!(result, false);
    }

    #[test]
    fn half_function_call_is_not_a_call_expression_statement() {
        let content = String::from("foobar(");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_call_expression_statement(&parser);

        assert_eq!(result, false);
    }

    #[test]
    fn function_call_without_params_is_parsable_call_expression() {
        let content = String::from("call()");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = find(&mut parser).unwrap().unwrap().value;
        let call_expression = cast_expression_statement!(result, Expression::CallExpression).unwrap();

        assert_eq!(call_expression.callee.name, "call");
        assert_eq!(call_expression.arguments.len(), 0);
    }

    #[test]
    fn function_call_with_params_is_parsable_call_expression() {
        let content = String::from("call('123', 123, call())");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = find(&mut parser).unwrap().unwrap().value;
        let call_expression = cast_expression_statement!(result, Expression::CallExpression).unwrap();

        assert_eq!(call_expression.callee.name, "call");
        assert_eq!(call_expression.arguments.len(), 3);
    }

    #[test]
    fn half_function_call_is_not_a_parsable_call_expression() {
        let content = String::from("call(");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = find(&mut parser).unwrap();

        assert_eq!(result.is_none(), true);
    }

}
