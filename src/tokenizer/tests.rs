use super::tokenizer::parse;

#[test]
fn parse_gives_back_single_tokens() {
    let content = String::from("'Hello world'");

    let tokens = parse(&content).unwrap();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens.get(0).unwrap().value, "Hello world");
}

#[test]
fn parse_gives_back_two_tokens() {
    let content = String::from("'Hello world' 'Hello world'");

    let tokens = parse(&content).unwrap();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens.get(0).unwrap().value, "Hello world");
    assert_eq!(tokens.get(1).unwrap().value, "Hello world");
}

#[test]
fn equation_num_plus_num_gives_back_correct_tokens() {
    let content = String::from("123 + 321");

    let tokens = parse(&content).unwrap();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens.get(0).unwrap().value, "123");
    assert_eq!(tokens.get(1).unwrap().value, "+");
    assert_eq!(tokens.get(2).unwrap().value, "321");    
}

#[test]
fn equation_num_min_num_gives_back_correct_tokens() {
    let content = String::from("123 - 321");

    let tokens = parse(&content).unwrap();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens.get(0).unwrap().value, "123");
    assert_eq!(tokens.get(1).unwrap().value, "-");
    assert_eq!(tokens.get(2).unwrap().value, "321");
}
