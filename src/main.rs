use std::fmt::write;
use std::{fmt, fs};
use std::error::Error;

// Compiler modules
mod lexer;
use lexer::*;

// Module for testing
mod tests;

// TODO:
// - Parser
// - Emitter

fn main() -> Result<(), Box<dyn Error>> {
    // Starts compiling when args[1] is the filename.
    let args: Vec<String> = std::env::args().collect();
    let source: String;

    if args.len() == 1 {
        panic!("provide filepath as an argument.");
    }
    else if args.len() > 2 {
        panic!("expected 1 argument: provide filepath as the only argument");
    }

    if let Ok(src) = fs::read_to_string(&args[1]) {
        source = src;
    } 
    else {
        panic!("could not find `{}` file or read from it.", args[1]);
    }

    let mut lexer = Lexer::new(source);
    let mut token: Token = lexer.get_token();

    while !matches!(token.kind, TokenType::EOF) {
        println!("{:?}", token.kind);
        token = lexer.get_token();
    }

    Ok(())
}


#[cfg(test)]
fn _tests() {
    use tests::*;

    // Lexer tests
    lexer_tests::operator_token_test();
    lexer_tests::keyword_token_test();
    // Special token testing will be implemented later when better error handling will be added.
    // lexer_tests::special_token_test();

    // Parser tests
}
