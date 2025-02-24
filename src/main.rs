use std::fs;
use std::error::Error;

// Compiler modules
mod lexer;
use lexer::*;

// Module for testing
mod tests;

// TODO:
// - Read from a file
// - Parser
// - Emitter

fn main() -> Result<(), Box<dyn Error>> {
    let source: String = fs::read_to_string("main.rebasic").expect("Could not load the specified file.");
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

    // Parser tests
}
