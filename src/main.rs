mod lexer;

use lexer::*;

// TODO:
// 1. Lexer (done)
// 2. Read from a file
// 3. Parser
// 4. Emitter

fn main() {
    let source = String::from("IF+-123 foo*THEN/");
    let mut lexer = Lexer::new(source);

    let mut token: Token = lexer.get_token();

    while !matches!(token.kind, TokenType::EOF) {
        println!("{:?}", token.kind);
        token = lexer.get_token();
    }
}
