use crate::{ast::{parser::AstParser, nodes::{expression_statement::{ExpressionStatement, IdentifierExpression, Expression}, Identifier}, AstParseError, AstErrorType, parsers::{block_statements::is_closed_block_statement, util::{is_open_param_bracket, is_closed_param_bracket}}, SearchResult}, tokenizer::{TokenType, Separator}, ast_error};

use super::{FindResult, consume_result};

pub fn is_identifier_expression_statement(parser: &AstParser) -> bool {
    // TODO: Maybe change this to only look for the most basic parts of a identifier (the first token)
    if let Ok(response) = find(parser) {
        return response.is_some();
    }

    false
}

pub fn find(parser: &AstParser) -> FindResult<ExpressionStatement> {
    let handle_identifier_token = |parser: &AstParser| -> Result<Identifier, AstParseError> {
        match parser.token() {
            Some(token) => {
                if !matches!(token.token_type, TokenType::Identifier) {
                    return ast_error!(AstErrorType::UnexpectedToken, parser);
                }

                Ok(Identifier::from(token))
            },
            None => ast_error!(AstErrorType::UnexpectedToken, parser)
        }
    };

    let check_if_identifier_expression_has_ended = |parser: &AstParser| -> bool {
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

                if is_open_param_bracket(parser, index) {
                    return true;
                }

                if is_closed_param_bracket(parser, index) {
                    return true;
                }
                
                if index > 0 && parser.can_insert_automatic_semicolon(index) {
                    return true;
                }
    
                return false
            },
            None => return true,
        }
    };

    let identifier = handle_identifier_token(parser)?;

    if !check_if_identifier_expression_has_ended(parser) {
        return ast_error!(AstErrorType::UnexpectedToken, parser);
    }

    let literal_start   = identifier.range.0;
    let literal_end     = identifier.range.1;

    let ast_start       = parser.get_current_index();
    let ast_end         = ast_start + 1;

    let expression = IdentifierExpression { identifier };

    let expression_statement = ExpressionStatement {
        expression: Expression::Identifier(expression),
        range: (literal_start, literal_end)
    };

    Ok(Some(SearchResult::<ExpressionStatement> {
        value: expression_statement,
        ast_range: (ast_start, ast_end),
    }))
}

pub fn parse_identifier_expression_statement(parser: &mut AstParser) -> Result<ExpressionStatement, AstParseError> {    
    let result = find(parser)?;

    if result.is_none() {
        return ast_error!(AstErrorType::UnexpectedToken, parser);
    }

    return Ok(consume_result(parser, result.unwrap()));
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer, ast::{parser::AstParser, nodes::expression_statement::Expression, parsers::expression_statements::{identifier_expression::is_identifier_expression_statement, consume_result}, AstErrorType}};

    use super::find;

    #[test]
    fn identifier_is_identifier_statement() {
        let content = String::from("foobar");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_identifier_expression_statement(&parser);

        assert_eq!(result, true);
    }

    #[test]
    fn string_is_not_a_identifier_statement() {
        let content = String::from("'Foobar'");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_identifier_expression_statement(&parser);

        assert_eq!(result, false);
    }

    
    #[test]
    fn number_is_not_a_identifier_statement() {
        let content = String::from("123");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_identifier_expression_statement(&parser);

        assert_eq!(result, false);
    }

    #[test]
    fn identifier_is_parsable_identifier_expression() {
        let content = String::from("foobar");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = find(&mut parser).unwrap().unwrap().value;

        if let Expression::Identifier(expression) = result.expression {
            let identifier = expression.identifier;
            assert_eq!(identifier.range.0, 0);
            assert_eq!(identifier.range.1, 6);
            assert_eq!(identifier.name, "foobar");
    
            assert_eq!(parser.get_current_index(), 0);
        } else {
            panic!("Invalid return value");
        }
    }

    #[test]
    fn multiple_wrong_separated_is_not_parsable_identifier_expression() {
        let content = String::from("foobar foo");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = find(&mut parser).unwrap_err();

        assert_eq!(result.error_type, AstErrorType::UnexpectedToken);
    }

    #[test]
    fn new_line_separated_strings_are_parsable_literal_expression() {
        let content = String::from("x\ny");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = find(&mut parser).unwrap().unwrap();
        consume_result(&mut parser, result);

        let result = find(&mut parser).unwrap().unwrap().value;

        if let Expression::Identifier(expression) = result.expression {
            let identifier = expression.identifier;
            assert_eq!(identifier.range.0, 2);
            assert_eq!(identifier.range.1, 3);
            assert_eq!(identifier.name, "y");
    
            assert_eq!(parser.get_current_index(), 1);
        } else {
            panic!("Invalid return value");
        }
    }
}