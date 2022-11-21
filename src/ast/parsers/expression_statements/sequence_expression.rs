use crate::{ast::parser::AstParser, tokenizer::TokenType};

use super::is_single_expression_statement;

fn first_statement_is_expression(parser: &AstParser) -> bool {
    is_single_expression_statement(parser)
}

fn comma_is_before_separator(parser: &AstParser) -> bool {
    false
}

pub fn is_sequence_expression_statement(parser: &AstParser) -> bool {
    // The first token should be an expression statement
    first_statement_is_expression(parser) && comma_is_before_separator(parser)
}

#[cfg(test)]
mod tests {
    use crate::{ast::{parsers::expression_statements::sequence_expression::is_sequence_expression_statement, parser::AstParser}, tokenizer};

    #[test]
    fn string_and_string_is_valid_sequence() {
        let content = String::from("'foo', 'bar'");
    
        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);
    
        let result = is_sequence_expression_statement(&parser);
    
        // assert_eq!(result, true);
    }

    #[test]
    fn func_and_string_is_valid_sequence() {
        let content = String::from("foo(x, y, z), 'bar'");
    
        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);
    
        let result = is_sequence_expression_statement(&parser);
    
        // assert_eq!(result, true);
    }
}
