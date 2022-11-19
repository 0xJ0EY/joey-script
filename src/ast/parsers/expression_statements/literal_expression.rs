use crate::{tokenizer::{TokenType, Separator}, ast::{parser::AstParser, AstParseError, nodes::{expression_statement::{ExpressionStatement, LiteralExpression, Expression}, Literal}, AstErrorType}};

pub fn is_literal_expression_statement(parser: &AstParser) -> bool {
    match parser.token() {
        Some(token) => matches!(token.token_type, TokenType::Literal(_)),
        None => false,
    }
}

pub fn parse_literal_expression_statement(parser: &mut AstParser) -> Result<ExpressionStatement, AstParseError> {
    let handle_literal_token = |parser: &mut AstParser| {
        match parser.token() {
            Some(token) => Ok(Literal::from(token)),
            None => return Err(AstParseError { index: parser.get_current_index(), error_type: AstErrorType::UnexpectedToken }),
        }
    };

    let handle_end_marker = |parser: &mut AstParser| {
        let end_marker = parser.token();

        if end_marker.is_some() {
            let end_marker = end_marker.unwrap();

            if !matches!(end_marker.token_type, TokenType::Separator(Separator::Terminator)) {
                // Check if we can insert an automatic semicolon
                let index_of_literal = parser.get_current_index() - 1;
                
                if !parser.can_insert_automatic_semicolon(index_of_literal) {
                    return Err(AstParseError { index: parser.get_current_index(), error_type: AstErrorType::UnexpectedToken });
                }
            } else {
                let end = end_marker.range.1.clone();
                parser.next();

                return Ok(Some(end));
            }
        }

        Ok(None)
    };
    
    let literal_token = handle_literal_token(parser)?;
    parser.next();

    let end_marker= handle_end_marker(parser)?;

    let start   = literal_token.range.0;
    let end     = end_marker.unwrap_or(literal_token.range.1);

    let expression = LiteralExpression { value: literal_token };

    Ok(ExpressionStatement {
        expression: Expression::Literal(expression),
        range: (start, end),
    })

}

#[cfg(test)]
mod tests {
    use crate::{tokenizer, ast::{parser::AstParser, nodes::expression_statement::Expression, parsers::expression_statements::literal_expression::is_literal_expression_statement}};

    use super::parse_literal_expression_statement;

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

        if let Expression::Literal(expression) = result.expression {
            let literal = expression.value;
            assert_eq!(literal.range.0, 0);
            assert_eq!(literal.range.1, 8);
            assert_eq!(literal.value, "Foobar");
    
            assert_eq!(parser.get_current_index(), 1);
        } else {
            panic!("Invalid return value");
        }
    }

    #[test]
    fn new_line_separated_strings_are_parsable_literal_expression() {
        let content = String::from("'Foobar'\n'Bar'");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        _ = parse_literal_expression_statement(&mut parser).unwrap();
        let result = parse_literal_expression_statement(&mut parser).unwrap();

        if let Expression::Literal(expression) = result.expression {
            let literal = expression.value;
            assert_eq!(literal.range.0, 9);
            assert_eq!(literal.range.1, 14);
            assert_eq!(literal.value, "Bar");
    
            assert_eq!(parser.get_current_index(), 2);
        } else {
            panic!("Invalid return value");
        }
    }

    #[test]
    fn both_strings_are_parsable_literal_expression() {
        let content = String::from("'Foobar';'Bar';");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        _ = parse_literal_expression_statement(&mut parser).unwrap();
        let result = parse_literal_expression_statement(&mut parser).unwrap();

        if let Expression::Literal(expression) = result.expression {
            let literal = expression.value;
            assert_eq!(literal.range.0, 9);
            assert_eq!(literal.range.1, 14);
            assert_eq!(literal.value, "Bar");
    
            assert_eq!(parser.get_current_index(), 4);
        } else {
            panic!("Invalid return value");
        }
    }

    #[test]
    fn three_strings_are_parsable_literal_expression() {
        let content = String::from("'Foobar';'Bar';'Foo';");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        _ = parse_literal_expression_statement(&mut parser).unwrap();
        _ = parse_literal_expression_statement(&mut parser).unwrap();
        let result = parse_literal_expression_statement(&mut parser).unwrap();

        if let Expression::Literal(expression) = result.expression {
            let literal = expression.value;
            assert_eq!(literal.range.0, 15);
            assert_eq!(literal.range.1, 20);
            assert_eq!(literal.value, "Foo");
    
            assert_eq!(parser.get_current_index(), 6);
        } else {
            panic!("Invalid return value");
        }
    }

}