use crate::{ast::{parser::AstParser, nodes::{expression_statement::{ExpressionStatement, Expression, CallExpression, SequenceExpression, self}, Identifier}, AstParseError, AstErrorType, parsers::{function_declaration::parse_function_declaration, util::{parse_function_name}, expression_statements::parse_expression_statement, parts::function_call::parse_function_call}, SearchResult}, tokenizer::TokenType, ast_error, handle_allowed_find_error};

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

