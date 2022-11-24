use crate::{ast::{parser::AstParser, nodes::expression_statement::{ExpressionStatement, IdentifierExpression, Expression}, AstParseError, AstErrorType, parsers::{util::{is_open_param_bracket, is_closed_param_bracket}, parts::identifier::parse_identifier}, SearchResult}, tokenizer::{TokenType, Separator}, ast_error};

use super::FindResult;

pub fn is_identifier_expression_statement(parser: &AstParser) -> bool {
    // TODO: Maybe change this to only look for the most basic parts of a identifier (the first token)
    if let Ok(response) = find(parser) {
        return response.is_some();
    }

    false
}

pub fn find(parser: &AstParser) -> FindResult<ExpressionStatement> {
    let start_index = parser.get_current_index();
    
    let check_if_identifier_expression_has_ended = |parser: &AstParser, start_index: usize| -> bool {
        let index = start_index + 1;
        let end_marker = parser.token_at(index);
    
        match end_marker {
            Some(marker) => {
                if matches!(marker.token_type, TokenType::Separator(Separator::Terminator)) {
                    return true;
                }
    
                if matches!(marker.token_type, TokenType::Separator(Separator::Comma)) {
                    return true;
                }

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

    let mut used_tokens = 0;
    let identifier_expression = parse_identifier(parser, start_index, &mut used_tokens)?;
    let identifier = identifier_expression.identifier;

    if !check_if_identifier_expression_has_ended(parser, start_index) {
        return ast_error!(AstErrorType::UnexpectedToken, parser);
    }

    let literal_start   = identifier.range.0;
    let literal_end     = identifier.range.1;

    let ast_start       = parser.get_current_index();
    let ast_end         = ast_start + used_tokens;

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