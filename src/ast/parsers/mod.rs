use crate::tokenizer::{TokenType, Separator};

use super::{parser::AstParser, AstParseError, nodes::{expression_statement::{ExpressionStatement, LiteralExpression, Expression}, Literal}};

pub fn is_literal_expression_statement(parser: &AstParser) -> bool {
    return match parser.token() {
        Some(token) => matches!(token.token_type, TokenType::Literal(_)),
        None => false,
    }
}

pub fn parse_literal_expression_statement(parser: &mut AstParser) -> Result<ExpressionStatement, AstParseError> {
    let handle_literal_token = |parser: &mut AstParser| {
        match parser.consume() {
            Some(token) => Ok(Literal::from(token)),
            None => return Err(AstParseError { index: parser.get_current_index(), error_type: super::AstErrorType::UnexpectedToken }),
        }
    };

    let handle_end_marker = |parser: &mut AstParser| {
        let end_marker = parser.consume();

        if end_marker.is_some() {
            let end_marker = end_marker.unwrap();
            if !matches!(end_marker.token_type, TokenType::Separator(Separator::Terminator)) {
                return Err(AstParseError { index: parser.get_current_index(), error_type: super::AstErrorType::UnexpectedToken });
            }
        }

        Ok(())
    };
    
    let literal_token = handle_literal_token(parser)?;
    let terminator_token = handle_end_marker(parser)?;

    let start = literal_token.range.0;
    let end = literal_token.range.1;

    let expression = LiteralExpression { value: literal_token };

    Ok(ExpressionStatement {
        expression: Expression::Literal(expression),
        range: (start, end),
    })

}

pub fn is_expression_statement(parser: &AstParser) -> bool {
    return is_literal_expression_statement(parser)
}

pub fn parse_expression_statement(parser: &mut AstParser) -> Result<ExpressionStatement, AstParseError> {
    if is_literal_expression_statement(parser) {
        return parse_literal_expression_statement(parser);
    }

    return Err(AstParseError { index: parser.get_current_index(), error_type: super::AstErrorType::UnexpectedToken });
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer, ast::{parser::AstParser, parsers::{is_literal_expression_statement, parse_literal_expression_statement}}};

    #[test]
    fn string_is_literal_expression_statement() {
        let content = String::from("'Foobar'");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_literal_expression_statement(&parser);

        assert_eq!(result, true);
    }

    #[test]
    fn number_is_literal_expression_statement() {
        let content = String::from("123");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_literal_expression_statement(&parser);

        assert_eq!(result, true);
    }

    #[test]
    fn null_is_literal_expression_statement() {
        let content = String::from("null");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_literal_expression_statement(&parser);

        assert_eq!(result, true);
    }

    #[test]
    fn boolean_is_literal_expression_statement() {
        let content = String::from("true");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_literal_expression_statement(&parser);

        assert_eq!(result, true);
    }

    #[test]
    fn keyword_is_not_literal_expression_statement() {
        let content = String::from("while");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_literal_expression_statement(&parser);

        assert_eq!(result, false);
    }

    #[test]
    fn string_is_parsable_literal_expression() {
        let content = String::from("'Foobar'");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = parse_literal_expression_statement(&mut parser).unwrap();
        /*
        let literal = result.expression.value;

        assert_eq!(literal.range.0, 0);
        assert_eq!(literal.range.1, 8);
        assert_eq!(literal.value, "Foobar");

        assert_eq!(parser.get_current_index(), 2);
        */
    }

    #[test]
    fn both_strings_are_parsable_literal_expression() {
        let content = String::from("'Foobar';'Bar';");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        _ = parse_literal_expression_statement(&mut parser).unwrap();
        let result = parse_literal_expression_statement(&mut parser).unwrap();
        /*
        let literal = result.value;

        assert_eq!(literal.range.0, 9);
        assert_eq!(literal.range.1, 14);
        assert_eq!(literal.value, "Bar");

        assert_eq!(parser.get_current_index(), 4);
        */ 
    }

    #[test]
    fn three_strings_are_parsable_literal_expression() {
        let content = String::from("'Foobar';'Bar';'Foo';");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        _ = parse_literal_expression_statement(&mut parser).unwrap();
        _ = parse_literal_expression_statement(&mut parser).unwrap();
        let result = parse_literal_expression_statement(&mut parser).unwrap();
        /*
        let literal = result.value;

        assert_eq!(literal.range.0, 15);
        assert_eq!(literal.range.1, 20);
        assert_eq!(literal.value, "Foo");

        assert_eq!(parser.get_current_index(), 6);
        */
    }

}