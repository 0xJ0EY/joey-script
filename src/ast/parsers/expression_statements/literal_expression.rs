use crate::{tokenizer::{TokenType, Separator}, ast::{parser::AstParser, AstParseError, nodes::{expression_statement::{ExpressionStatement, LiteralExpression, Expression}, Literal}, AstErrorType, parsers::{block_statements::is_closed_block_statement, parts::literal::parse_literal}, SearchResult}, ast_error, handle_allowed_find_error};

use super::FindResult;

pub fn is_literal_expression_statement(parser: &AstParser) -> bool {
    // TODO: Maybe change this to only look for the most basic parts of a literal (the first token)
    if let Ok(response) = find(parser) {
        return response.is_some();
    }

    false
}

pub fn find(parser: &AstParser) -> FindResult<ExpressionStatement> {
    let check_if_expression_has_ended = |parser: &AstParser| -> bool {
        let end_marker = parser.peek();

        match end_marker {
            Some(marker) => {
                if matches!(marker.token_type, TokenType::Separator(Separator::Terminator)) {
                    return true;
                }
    
                if matches!(marker.token_type, TokenType::Separator(Separator::Comma)) {
                    return true;
                }
    
                let index = parser.get_current_index() + 1;
                if index > 0 && parser.can_insert_automatic_semicolon(index) {
                    return true;
                }
    
                return false
            },
            None => return true,
        }
    };
    
    let token = handle_allowed_find_error!(parse_literal(parser));
    
    if !check_if_expression_has_ended(parser) {
        return ast_error!(AstErrorType::UnexpectedToken, parser);
    }

    let literal_start   = token.range.0;
    let literal_end     = token.range.1;

    let ast_start       = parser.get_current_index();
    let ast_end         = ast_start + 1;

    let expression = LiteralExpression { value: token };

    let expression_statement = ExpressionStatement {
        expression: Expression::Literal(expression),
        range: (literal_start, literal_end),
    };

    Ok(Some(SearchResult::<ExpressionStatement> {
        value: expression_statement,
        ast_range: (ast_start, ast_end)
    }))
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer, ast::{parser::AstParser, nodes::expression_statement::Expression, parsers::expression_statements::{literal_expression::is_literal_expression_statement, consume_result}}};

    use super::find;

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

        let result = find(&mut parser).unwrap().unwrap().value;

        if let Expression::Literal(expression) = result.expression {
            let literal = expression.value;
            assert_eq!(literal.range.0, 0);
            assert_eq!(literal.range.1, 8);
            assert_eq!(literal.value, "Foobar");
    
            assert_eq!(parser.get_current_index(), 0);
        } else {
            panic!("Invalid return value");
        }
    }

    #[test]
    fn new_line_separated_strings_are_parsable_literal_expression() {
        let content = String::from("'Foobar'\n'Bar'");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = find(&mut parser).unwrap().unwrap();
        consume_result(&mut parser, result);

        let result = find(&mut parser).unwrap().unwrap().value;

        if let Expression::Literal(expression) = result.expression {
            let literal = expression.value;
            assert_eq!(literal.range.0, 9);
            assert_eq!(literal.range.1, 14);
            assert_eq!(literal.value, "Bar");
    
            assert_eq!(parser.get_current_index(), 1);
        } else {
            panic!("Invalid return value");
        }
    }

    #[test]
    fn both_strings_are_parsable_literal_expression() {
        let content = String::from("'Foobar';'Bar';");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = find(&mut parser).unwrap().unwrap();
        consume_result(&mut parser, result);
        parser.next(); // Skip the ;

        let result = find(&mut parser).unwrap().unwrap().value;

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
    fn three_strings_are_parsable_literal_expression() {
        let content = String::from("'Foobar';'Bar';'Foo';");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = find(&mut parser).unwrap().unwrap();
        consume_result(&mut parser, result);

        parser.next(); // Skip the ;

        let result = find(&mut parser).unwrap().unwrap();
        consume_result(&mut parser, result);

        parser.next(); // Skip the ;

        let result = find(&mut parser).unwrap().unwrap().value;

        if let Expression::Literal(expression) = result.expression {
            let literal = expression.value;
            assert_eq!(literal.range.0, 15);
            assert_eq!(literal.range.1, 20);
            assert_eq!(literal.value, "Foo");
    
            assert_eq!(parser.get_current_index(), 4);
        } else {
            panic!("Invalid return value");
        }
    }

}