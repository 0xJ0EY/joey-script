use crate::tokenizer::{TokenType, Separator};

use super::{parser::AstParser, AstError, nodes::expression_statement::ExpressionStatement};

pub struct SearchResult { 
    start: usize,
    end: usize,
    delta: usize,
}

pub fn search_literal_expression_statement(parser: &AstParser) -> Option<SearchResult> {
    let start = parser.get_current_index();
    let mut delta = 0;

    let literal_token = match parser.peek_forward(0) {
        Some(token) => token,
        None => return None,
    };

    if !matches!(literal_token.token_type, TokenType::Literal(_)) {
       return None;
    }

    delta += 1;

    let end_marker = parser.peek_forward(1);

    if end_marker.is_some() {
        let end_marker = end_marker.unwrap();
        if !matches!(end_marker.token_type, TokenType::Separator(Separator::Terminator)) {
            return None;
        }
    }

    delta += 1;
    
    let end = start + delta;
    
    Some(SearchResult { start, end, delta })
}


pub fn parse_literal_expression_statement(parser: &AstParser) -> Result<ExpressionStatement, AstError> {
    todo!()
}


#[cfg(test)]
mod tests {
    use crate::{tokenizer, ast::parser::AstParser};

    use super::search_literal_expression_statement;

    #[test]
    fn find_literal_expression_statement_in_literal() {
        let content = String::from("'Foobar'");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = search_literal_expression_statement(&parser).unwrap();

        assert_eq!(result.start, 0);
        assert_eq!(result.end, 2);
        assert_eq!(result.delta, 2);
    }

    #[test]
    fn find_literal_expression_statement_in_literal_terminator() {
        let content = String::from("'Foobar';");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        let result = search_literal_expression_statement(&parser).unwrap();

        assert_eq!(result.start, 0);
        assert_eq!(result.end, 2);
        assert_eq!(result.delta, 2);
    }

    #[test]
    fn find_literal_expression_statement_in_literal_terminator_twice() {
        let content = String::from("'Foobar';'Barfoo';");

        let tokens = tokenizer::parse(&content).unwrap();
        let parser = AstParser::new(&tokens);

        _ = search_literal_expression_statement(&parser).unwrap();

        let result = search_literal_expression_statement(&parser).unwrap();

        assert_eq!(result.start, 2);
        assert_eq!(result.end, 4);
        assert_eq!(result.delta, 4);
    }

}