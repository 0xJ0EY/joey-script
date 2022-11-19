use crate::{ast::{AstParseError, parser::AstParser, nodes::{function_declaration::FunctionDeclaration, expression_statement::Expression, Identifier, block_statement::BlockStatement}, AstErrorType, parsers::block_statements::parse_block_statement}, tokenizer::{TokenType, Separator}, ast_error};

use super::{get_start_position, get_end_position_of_previous_token, expression_statements::{identifier_expression::parse_identifier_expression_statement}};

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

fn is_open_param_bracket(parser: &mut AstParser) -> bool {
    match parser.token() {
        Some(token) => {
            let is_type = matches!(token.token_type, TokenType::Separator(Separator::Parenthesis));
            let is_value = token.value == "(";

            is_type && is_value
        },
        None => false,
    }
}

fn is_param_separator(parser: &mut AstParser) -> bool {
    match parser.token() {
        Some(token) => matches!(token.token_type, TokenType::Separator(Separator::Comma)),
        None => false,
    }
}

fn is_closed_param_bracket(parser: &mut AstParser) -> bool {
    match parser.token() {
        Some(token) => {
            let is_type = matches!(token.token_type, TokenType::Separator(Separator::Parenthesis));
            let is_value = token.value == ")";

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

    let parse_function_name = |parser: &mut AstParser| -> Result<Identifier, AstParseError> {
        let expression = parse_identifier_expression_statement(parser)?;

        match expression.expression {
            Expression::Identifier(id) => Ok(id.identifier),
            _ => return ast_error!(AstErrorType::UnexpectedToken, parser)
        }
    };

    let parse_parameters = |parser: &mut AstParser| -> Result<Vec<Identifier>, AstParseError> {
        if !is_open_param_bracket(parser) { return ast_error!(AstErrorType::UnexpectedToken, parser) }
        let mut params = Vec::<Identifier>::new();

        // Skip opening bracket
        parser.next();

        // Exit early if we don't have any params
        if is_closed_param_bracket(parser) {
            parser.next();
            return Ok(params);
        }

        loop {
            // Validate if we got an identifier expression
            let expression_statement = parse_identifier_expression_statement(parser)?;
            // let node = AstNode::ExpressionStatement(param);

            let identifier = match expression_statement.expression {
                Expression::Identifier(id) => id.identifier,
                _ => return ast_error!(AstErrorType::UnexpectedToken, parser)
            };

            params.push(identifier);

            // Validate if we have a closing bracket, if so close the loop
            if is_closed_param_bracket(parser) {
                parser.next();
                break;
            }

            // Validate if we have "," separator
            if is_param_separator(parser) {
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

    // 1. Parse the function keyword
    _ = parse_function_keyword(parser)?;

    // 2. Parse the function name
    let function_name = parse_function_name(parser)?;

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

