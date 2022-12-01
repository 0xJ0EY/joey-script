use crate::{ast::{AstParseError, parser::AstParser, nodes::expression_statement::CallExpression, parsers::{parts::sequence::parse_sequence}, AstErrorType}, ast_error, tokenizer::{TokenType, Separator}};

use super::identifier::parse_identifier;

fn is_start_parenthesis(parser: &AstParser, index: usize) -> bool {
    match parser.token_at(index) {
        Some(token) => {
            if matches!(token.token_type, TokenType::Separator(Separator::Parenthesis)) {
                return token.value == "(";
            }

            false
        },
        None => false,
    }
}

fn is_end_parenthesis(parser: &AstParser, index: usize) -> bool {
    match parser.token_at(index) {
        Some(token) => {
            if matches!(token.token_type, TokenType::Separator(Separator::Parenthesis)) {
                return token.value == ")";
            }

            false
        },
        None => false,
    }
}

pub fn parse_function_call(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<CallExpression, AstParseError> {
    let mut tokens = 0;

    let function_name = parse_identifier(parser, index + tokens, &mut tokens)?;

    // Check for start param
    if !is_start_parenthesis(parser, index + tokens) {
        return ast_error!(AstErrorType::UnexpectedTokenStart, parser);
    } else {
        tokens += 1;
    }

    let arguments;

    if !is_end_parenthesis(parser, index + tokens) {
        arguments = parse_sequence(parser, index + tokens, &mut tokens)?.expressions;

        if !is_end_parenthesis(parser, index + tokens) {
            return ast_error!(AstErrorType::UnexpectedToken, parser);
        } else {
            tokens += 1;
        }
    } else {
        arguments = Vec::new();
        tokens += 1;
    }

    *tokens_used += tokens;

    let call_expression = CallExpression {
        callee: function_name.identifier,
        arguments,
    };

    Ok(call_expression)
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer, ast::{parser::AstParser}};

    use super::parse_function_call;

    #[test]
    fn function_call_is_function_call() {
        let content = String::from("foobar()");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let mut tokens_used = 0;

        let result = parse_function_call(&parser, 0, &mut tokens_used).unwrap();

        assert_eq!(result.callee.name, "foobar");
    }

    #[test]
    fn function_call_with_params_is_function_call() {
        let content = String::from("foobar('Foobar', 123, id)");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let mut tokens_used = 0;

        let result = parse_function_call(&parser, 0, &mut tokens_used).unwrap();

        assert_eq!(result.callee.name, "foobar");
    }

}
