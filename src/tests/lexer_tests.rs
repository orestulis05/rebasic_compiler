use crate::lexer::*; 

#[test]
pub fn operator_token_test(){
    
    let source = "+ - * / = == != > >= < <=".to_string();

    let mut correct_order = vec![
        TokenType::PLUS,
        TokenType::MINUS,
        TokenType::ASTERISK,
        TokenType::SLASH,
        TokenType::EQ,
        TokenType::EQEQ,
        TokenType::NOTEQ,
        TokenType::GT,
        TokenType::GTEQ,
        TokenType::LT,
        TokenType::LTEQ,
        
        // Additional new line is always added at the bottom
        TokenType::NEWLINE
    ];

    let mut lexer = Lexer::new(source);
    
    let mut token: Token = lexer.get_token();
    while !matches!(token.kind, TokenType::EOF) {
        assert_eq!(&correct_order.remove(0), &token.kind);
        
        token = lexer.get_token();
    }
}

#[test]
pub fn keyword_token_test(){

    let source = "LABEL GOTO PRINT INPUT LET IF THEN ENDIF WHILE REPEAT ENDWHILE".to_string();
    
    let mut correct_order = vec![
        TokenType::LABEL,
        TokenType::GOTO,
        TokenType::PRINT,
        TokenType::INPUT,
        TokenType::LET,
        TokenType::IF,
        TokenType::THEN,
        TokenType::ENDIF,
        TokenType::WHILE,
        TokenType::REPEAT,
        TokenType::ENDWHILE,

        // Additional new line is always added at the bottom
        TokenType::NEWLINE
    ];

    let mut lexer = Lexer::new(source);
    
    let mut token: Token = lexer.get_token();
    while !matches!(token.kind, TokenType::EOF) {
        assert_eq!(&correct_order.remove(0), &token.kind);
        
        token = lexer.get_token();
    }
}

// Implement later
#[test]
pub fn special_token_test(){
    
    // UNKNOWN = -2,
    // NEWLINE = 0,
    // NUMBER = 1,
    // IDENT = 2,
    // STRING = 3,

    let source = "` \n 10 10.23 identifier \"string\" ".to_string();

    let mut correct_order = vec![
        TokenType::UNKNOWN,
        TokenType::NEWLINE,
        TokenType::NUMBER,
        TokenType::IDENT,
        TokenType::STRING,
        
        // Additional new line is always added at the bottom
        TokenType::NEWLINE
    ];

    let mut lexer = Lexer::new(source);
    
    let mut token: Token = lexer.get_token();
    while !matches!(token.kind, TokenType::EOF) {
        assert_eq!(&correct_order.remove(0), &token.kind);      
        token = lexer.get_token();
    }
}

