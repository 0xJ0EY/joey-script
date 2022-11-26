
use crate::{ast::{parser::AstParser, nodes::expression_statement::{Expression, SequenceExpression}, AstParseError, AstErrorType}, ast_error, tokenizer::{TokenType, Separator}};

use super::{literal::parse_literal, identifier::parse_identifier, function_call::parse_function_call};

fn parse_sequence_token(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<Expression, AstParseError> {
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

fn is_end_marker(parser: &AstParser, index: usize, tokens_used: &mut usize) -> bool {
    match parser.token_at(index) {
        Some(token) => {
            if matches!(token.token_type, TokenType::Separator(Separator::CurlyBrace)) {
                return token.value == "}";
            }

            if matches!(token.token_type, TokenType::Separator(Separator::Parenthesis)) {
                return token.value == ")";
            }

            if matches!(token.token_type, TokenType::Separator(Separator::Terminator)) {
                *tokens_used += 1;

                return true;
            }

            false
        },
        None => true,
    }
}

fn parse_separator(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<(),  AstParseError> {
    match parser.token_at(index) {
        Some(token) => {
            if matches!(token.token_type, TokenType::Separator(Separator::Comma)) {
                *tokens_used += 1;

                return Ok(())
            }

            ast_error!(AstErrorType::UnexpectedToken, parser)
        },
        None => ast_error!(AstErrorType::UnexpectedToken, parser),
    }
}

pub fn parse_sequence(parser: &AstParser, index: usize, tokens_used: &mut usize) -> Result<SequenceExpression, AstParseError> {
    let mut tokens = 0;
    let mut expressions = Vec::new();

    loop {
        expressions.push(parse_sequence_token(parser, index + tokens, &mut tokens)?);

        if is_end_marker(parser, index + tokens, &mut tokens) { break; }

        parse_separator(parser, index + tokens, &mut tokens)?;
    }

    *tokens_used += tokens;

    let expression = SequenceExpression { expressions };

    Ok(expression)
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer, ast::{parser::AstParser, parsers::parts::sequence::parse_sequence, nodes::expression_statement::Expression}, cast_expression};

    #[test]
    fn sequence_can_be_parsed_as_a_sequence() {
        let content = String::from("'Foobar', Bar");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let mut tokens_used = 0;

        let result = parse_sequence(&parser, 0, &mut tokens_used).unwrap();

        let result1 = match result.expressions.get(0).unwrap() {
           Expression::Literal(val) => Some(&val.value),
            _ => None
        };

        let result2 =  match result.expressions.get(1).unwrap() {
            Expression::Identifier(val) => Some(&val.identifier),
             _ => None
        };

        assert_eq!(result.expressions.len(), 2);
        assert_eq!(tokens_used, 3);
        assert_eq!(result1.unwrap().value, "Foobar");
        assert_eq!(result2.unwrap().name, "Bar");
    }

}
