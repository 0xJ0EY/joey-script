use crate::{ast::{AstParseError, parser::AstParser, nodes::{function_declaration::FunctionDeclaration, expression_statement::Expression, Identifier, block_statement::BlockStatement}, AstErrorType, parsers::{block_statements::parse_block_statement, util::{is_closed_param_bracket, is_param_separator}}}, tokenizer::{TokenType, Separator}, ast_error};

use super::{get_start_position, get_end_position_of_previous_token, expression_statements::{identifier_expression::parse_identifier_expression_statement}, util::{parse_function_name, is_open_param_bracket}};

pub fn is_function_declaration(parser: &AstParser) -> bool {
    match parser.token() {
        Some(token) => {
            let is_type = matches!(token.token_type, TokenType::Keyword);
            let is_value = token.value == "function";

            is_type && is_value
        },
        None => false,
    }
}

pub fn parse_function_declaration(parser: &mut AstParser) -> Result<FunctionDeclaration, AstParseError> {

    let parse_function_keyword = |parser: &mut AstParser| -> Result<(), AstParseError> {
        if !is_function_declaration(parser) { return ast_error!(AstErrorType::UnexpectedToken, parser) }

        parser.next();

        Ok(())
    };

    let parse_parameters = |parser: &mut AstParser| -> Result<Vec<Identifier>, AstParseError> {
        if !is_open_param_bracket(parser, parser.get_current_index()) { return ast_error!(AstErrorType::UnexpectedToken, parser) }
        let mut params = Vec::<Identifier>::new();

        // Skip opening bracket
        parser.next();

        // Exit early if we don't have any params
        if is_closed_param_bracket(parser, parser.get_current_index()) {
            parser.next();
            return Ok(params);
        }

        loop {
            // Validate if we got an identifier expression
            let expression_statement = parse_identifier_expression_statement(parser)?;

            dbg!(&expression_statement);

            let identifier = match expression_statement.expression {
                Expression::Identifier(id) => id.identifier,
                _ => return ast_error!(AstErrorType::UnexpectedToken, parser)
            };

            params.push(identifier);

            // Validate if we have a closing bracket, if so close the loop
            if is_closed_param_bracket(parser, parser.get_current_index()) {
                parser.next();
                break;
            }

            // Validate if we have "," separator
            if is_param_separator(parser, parser.get_current_index()) {
                parser.next();
                continue;
            }
            
            return ast_error!(AstErrorType::UnexpectedToken, parser)
        }

        Ok(params)
    };

    let parse_block_scope = |parser: &mut AstParser| -> Result<BlockStatement, AstParseError> {
        parse_block_statement(parser)
    };

    let start = get_start_position(parser)?;

    dbg!(start);

    // 1. Parse the function keyword
    _ = parse_function_keyword(parser)?;

    // 2. Parse the function name
    let function_name = parse_function_name(parser)?;

    dbg!(&function_name);

    // 3. Parse the parameters
    let params = parse_parameters(parser)?;

    // 4. Parse the block scope
    let body = parse_block_scope(parser)?;
    let end = get_end_position_of_previous_token(parser)?;

    Ok(FunctionDeclaration {
        id: function_name,
        params,
        body,
        range: (start, end)
    })
}

#[cfg(test)]
mod tests {
    use crate::{tokenizer, ast::{parser::AstParser, AstErrorType}};

    use super::{is_function_declaration, parse_function_declaration};

    #[test]
    fn function_keyword_is_start_function_declaration() {
        let content = String::from("function");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_function_declaration(&parser);

        assert_eq!(result, true);
    }
    
    #[test]
    fn string_is_not_a_start_function_declaration() {
        let content = String::from("'Foobar'");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = is_function_declaration(&parser);

        assert_eq!(result, false);
    }

    #[test]
    fn valid_function_parses_as_a_function() {
        let content = String::from("function x() {}");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = parse_function_declaration(&mut parser).unwrap();

        assert_eq!(result.id.name, "x");
        assert_eq!(result.params.len(), 0);
    }

    #[test]
    fn valid_function_with_params_parses_as_a_function() {
        let content = String::from("function x(x, y, z) {}");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = parse_function_declaration(&mut parser).unwrap();

        assert_eq!(result.id.name, "x");
        assert_eq!(result.params.len(), 3);
    }

    #[test]
    fn not_valid_function_gives_an_error() {
        let content = String::from("function x(123) {}");

        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);

        let result = parse_function_declaration(&mut parser).unwrap_err();

        assert_eq!(result.error_type, AstErrorType::UnexpectedToken);
    }

}