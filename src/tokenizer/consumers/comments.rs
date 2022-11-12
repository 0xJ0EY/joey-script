use crate::tokenizer::{tokenizer::Tokenizer, util::is_eol};

fn is_start_line_comment(tokenizer: &Tokenizer) -> bool {
    let first   = tokenizer.token().unwrap();
    let second  = tokenizer.peek().unwrap_or(&' ');

    *first == '/' && *second == '/'
}

fn is_end_line_comment(tokenizer: &Tokenizer) -> bool {
    is_eol(tokenizer.token().unwrap_or(&' '))
}

pub fn is_line_comment(tokenizer: &Tokenizer) -> bool {
    is_start_line_comment(tokenizer)
}

pub fn consume_line_comment(tokenizer: &mut Tokenizer) {
    if !is_start_line_comment(tokenizer) { return; }

    let mut consumed = tokenizer.token();

    while consumed.is_some() && !is_end_line_comment(tokenizer) {

        tokenizer.next();
        consumed = tokenizer.token();
    }

    if is_end_line_comment(tokenizer) {
        tokenizer.next(); // Consume new line
    }
}

fn is_start_block_comment(tokenizer: &Tokenizer) -> bool {
    let first   = tokenizer.token().unwrap();
    let second  = tokenizer.peek().unwrap_or(&' ');

    *first == '/' && *second == '*'
}

fn is_end_block_comment(tokenizer: &mut Tokenizer) -> bool {
    
    let first   = tokenizer.token().unwrap_or(&' ');
    let second  = tokenizer.peek().unwrap_or(&' ');

    *first == '*' && *second == '/'
}

pub fn is_block_comment(tokenizer: &Tokenizer) -> bool {
    is_start_block_comment(tokenizer)
}

pub fn consume_block_comment(tokenizer: &mut Tokenizer) {
    if !is_start_block_comment(tokenizer) { return; }

    let mut consumed = tokenizer.token();
    println!("{:?}", consumed);

    while consumed.is_some() && !is_end_block_comment(tokenizer) {
        tokenizer.next();
        consumed = tokenizer.token();
    }    

    if is_end_block_comment(tokenizer) {
        tokenizer.next(); // Consume last *
        tokenizer.next(); // Consume last /         
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tokenizer::tokenizer::Tokenizer;

    #[test]
    fn is_start_line_comment() {
        let input = String::from_str("// Foobar").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_line_comment(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn is_start_block_comment() {
        let input = String::from_str("/* Foobar */").unwrap();
        let tokenizer = Tokenizer::new(&input);

        let result = super::is_block_comment(&tokenizer);

        assert_eq!(result, true);
    }

    #[test]
    fn process_line_comment() {
        let input = String::from_str("// Foobar").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        super::consume_line_comment(&mut tokenizer);

        assert_eq!(tokenizer.get_current_index(), input.len());
    }

    #[test]
    fn process_line_comment_with_newline() {
        let input = String::from_str("// Foo\n//Bar").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        super::consume_line_comment(&mut tokenizer);

        assert_eq!(tokenizer.get_current_index(), 7);
    }

    #[test]
    fn process_block_comment() {
        let input = String::from_str("/* Foobar */").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        super::consume_block_comment(&mut tokenizer);

        assert_eq!(tokenizer.get_current_index(), input.len());
    }

    #[test]
    fn process_block_comment_without_end_block() {
        let input = String::from_str("/* Foobar").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        super::consume_block_comment(&mut tokenizer);

        assert_eq!(tokenizer.get_current_index(), input.len());
    }

    #[test]
    fn process_block_comment_with_broken_end_block() {
        let input = String::from_str("/* Foobar *").unwrap();
        let mut tokenizer = Tokenizer::new(&input);

        super::consume_block_comment(&mut tokenizer);

        assert_eq!(tokenizer.get_current_index(), input.len());
    }

}
