use crate::{ast::{parser::AstParser, nodes::{expression_statement::{ExpressionStatement, IdentifierExpression, Expression}, Identifier}, AstParseError, AstErrorType}, tokenizer::{TokenType}, ast_error};

pub fn is_identifier_expression_statement(parser: &AstParser) -> bool {
    match parser.token() {
        Some(token) => matches!(token.token_type, TokenType::Identifier),
        None => false,
    }
}

pub fn parse_identifier_expression_statement(parser: &mut AstParser) -> Result<ExpressionStatement, AstParseError> {
    if !is_identifier_expression_statement(parser) { return ast_error!(AstErrorType::UnexpectedToken, parser) }

    let handle_identifier_token = |parser: &mut AstParser| -> Result<Identifier, AstParseError> {
        match parser.token() {
            Some(token) => Ok(Identifier::from(token)),
            None => ast_error!(AstErrorType::UnexpectedToken, parser)
        }
    };

    let identifier = handle_identifier_token(parser)?;
    
    parser.next();

    let start   = identifier.range.0;
    let end     = identifier.range.1;

    let expression = IdentifierExpression { identifier };

    Ok(ExpressionStatement {
        expression: Expression::Identifier(expression),
        range: (start, end)
    })
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer, ast::{parser::AstParser, nodes::expression_statement::Expression, parsers::expression_statements::identifier_expression::is_identifier_expression_statement}};

    use super::parse_identifier_expression_statement;

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

        let result = parse_identifier_expression_statement(&mut parser).unwrap();

        if let Expression::Identifier(expression) = result.expression {
            let identifier = expression.identifier;
            assert_eq!(identifier.range.0, 0);
            assert_eq!(identifier.range.1, 6);
            assert_eq!(identifier.name, "foobar");
    
            assert_eq!(parser.get_current_index(), 1);
        } else {
            panic!("Invalid return value");
        }
    }

    #[test]
    fn new_line_separated_strings_are_parsable_literal_expression() {
        let content = String::from("x\ny");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        _ = parse_identifier_expression_statement(&mut parser).unwrap();
        let result = parse_identifier_expression_statement(&mut parser).unwrap();

        if let Expression::Identifier(expression) = result.expression {
            let identifier = expression.identifier;
            assert_eq!(identifier.range.0, 2);
            assert_eq!(identifier.range.1, 3);
            assert_eq!(identifier.name, "y");
    
            assert_eq!(parser.get_current_index(), 2);
        } else {
            panic!("Invalid return value");
        }
    }
}