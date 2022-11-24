use crate::{ast::{parser::AstParser, nodes::expression_statement::{SequenceExpression, self, ExpressionStatement, Expression}, parsers::parts::sequence::parse_sequence, AstErrorType, AstParseError, SearchResult}, tokenizer::{TokenType, Separator}, ast_error};

use super::FindResult;

pub fn is_sequence_expression_statement(parser: &AstParser) -> bool {
    // The first token should be an expression statement
    if let Ok(response) = find(parser) {
        return response.is_some();
    }

    false
}

pub fn find(parser: &AstParser) -> FindResult<ExpressionStatement> {
    let start_index = parser.get_current_index();

    let check_if_sequence_has_ended = |parser: &AstParser, start_index: usize| -> bool {
        let index = start_index + 1;
        let end_marker = parser.token_at(index);

        match end_marker {
            Some(marker) => {
                if matches!(marker.token_type, TokenType::Separator(Separator::Terminator)) {
                    return true;
                }

                false
            },
            None => true,
        }
    };

    let mut used_tokens = 0;
    let sequence = parse_sequence(parser, start_index, &mut used_tokens)?;

    if !check_if_sequence_has_ended(parser, start_index + used_tokens) {
        return ast_error!(AstErrorType::UnexpectedToken, parser);
    }

    // Add end marker
    used_tokens += 1;

    // TOD: Implement correct literal distance
    let literal_start = 0;
    let literal_end = 0;

    let ast_start = start_index;
    let ast_end = ast_start + used_tokens;

    let expression = SequenceExpression { expressions: sequence };

    let expression_statement = ExpressionStatement {
        expression: Expression::SequenceExpression(expression),
        range: (literal_start, literal_end),
    };

    Ok(Some(SearchResult::<ExpressionStatement> {
        value: expression_statement,
        ast_range: (ast_start, ast_end),
    }))
}

#[cfg(test)]
mod tests {
    use crate::{ast::{parsers::expression_statements::sequence_expression::{is_sequence_expression_statement, find}, parser::AstParser, nodes::expression_statement::{ExpressionStatement, Expression}}, tokenizer, cast_expression_statement};

    #[test]
    fn string_and_string_is_valid_sequence_statement() {
        let content = String::from("'foo', 'bar';");
    
        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);
    
        let result = is_sequence_expression_statement(&parser);
    
        assert_eq!(result, true);
    }

    #[test]
    fn string_and_identifier_is_valid_sequence_statement() {
        let content = String::from("'foo', 123;");
    
        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);
    
        let result = is_sequence_expression_statement(&parser);
    
        assert_eq!(result, true);
    }

    #[test]
    fn string_and_identifier_without_terminator_is_valid_sequence_statement() {
        let content = String::from("'foo', 123");
    
        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);
    
        let result = is_sequence_expression_statement(&parser);
    
        assert_eq!(result, true);
    }

    // #[test]
    // fn func_and_string_is_valid_sequence() {
    //     let content = String::from("foo(x, y, z), 'bar'");
    
    //     let tokens = tokenizer::parse(&content).unwrap();
    //     let parser = AstParser::new(&tokens);
    
    //     let result = is_sequence_expression_statement(&parser);
    
    //     assert_eq!(result, true);
    // }

    #[test]
    fn string_and_identifier_is_parsable_sequence_expression() {
        let content = String::from("'foo', 'bar';");
    
        let tokens = tokenizer::parse(&content).unwrap();
        let mut parser = AstParser::new(&tokens);
    
        let result = find(&mut parser).unwrap().unwrap().value;
        let sequence = cast_expression_statement!(result, Expression::SequenceExpression).unwrap();

        assert_eq!(sequence.expressions.len(), 2);
    }

}
