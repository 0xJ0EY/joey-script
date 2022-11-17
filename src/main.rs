use core::panic;

mod tokenizer;
mod ast;

fn main() {
    let input = String::from(include_str!("input.js"));
    let tokens = tokenizer::parse(&input);

    println!("{:?}", tokens);

    // TODO: Temporary error handling
    if tokens.is_err() { panic!("Invalid tokens"); }

    let program = ast::parse(&tokens.unwrap());

    println!("{:?}", program);
}
