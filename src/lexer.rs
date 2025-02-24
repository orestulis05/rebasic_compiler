use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, PartialOrd, Clone, Copy)]
pub enum TokenType{
    UNKNOWN = -2,
    EOF = -1,
	NEWLINE = 0,
	NUMBER = 1,
	IDENT = 2,
	STRING = 3,
	// Keywords.
	LABEL = 101,
	GOTO = 102,
	PRINT = 103,
	INPUT = 104,
	LET = 105,
	IF = 106,
	THEN = 107,
	ENDIF = 108,
	WHILE = 109,
	REPEAT = 110,
	ENDWHILE = 111,
	// Operators.
	EQ = 201,  
	PLUS = 202,
	MINUS = 203,
	ASTERISK = 204,
	SLASH = 205,
	EQEQ = 206,
	NOTEQ = 207,
	LT = 208,
	LTEQ = 209,
	GT = 210,
	GTEQ = 211
}



pub struct Token {
    pub text: String,
    pub kind: TokenType
}

impl Token {
    pub fn new(text: String, kind: TokenType) -> Self{
        return Self {text, kind};
    }

    pub fn empty() -> Self{
        return Self {text: "".to_string(), kind: TokenType::UNKNOWN};
    }

    pub fn check_if_keyword(token_text: &str) -> TokenType{
        for kind in TokenType::iter(){
            let kind_name = format!("{:?}", kind);
            
            // println!("{}?:{}, number: {}", &kind_name, token_text, kind as i32);

            if &kind_name == token_text && kind as i32 >= 100 && kind as i32 <= 199 {
                return kind;
            }
        }

        return TokenType::IDENT;
    }
}

pub struct Lexer {
    pub source: String,
    pub current_char: char,
    pub current_pos: i64, 
    pub current_line: usize
}

impl Lexer {
    pub fn new(source: String) -> Self{
        let mut n = Self{
            source: format!("{}\n", source),
            current_char: '-',
            current_pos: -1,
            current_line: 1
        };

        n.next_char();
        return n;
    }

    pub fn abort(&self, message: String){
        println!("Lexing error on line {}: {}", self.current_line, message);
        std::process::exit(0)
    }

    pub fn next_char(&mut self){
        self.current_pos += 1;
        if self.current_pos as usize >= self.source.len(){
            self.current_char = '\0';
        }
        else {
            self.current_char = self.source.chars().nth(self.current_pos as usize).unwrap();
        }
    }

    // Returns the next character that should be examined in the source.
    pub fn peek(&self) -> char {
        if self.current_pos + 1 >= self.source.len() as i64{
            return '\0';
        } 
        else {
            return self.source.chars().nth(self.current_pos as usize + 1).unwrap();
        }
    }

    pub fn skip_whitespace(&mut self){
        while self.current_char == ' ' || self.current_char == '\t' || self.current_char == '\r'{
            self.next_char();
        }
    }

    pub fn skip_comment(&mut self){
        if self.current_char == '#'{
            while self.current_char != '\n'{
                self.next_char();
            }
        }
    }

    pub fn get_token(&mut self) -> Token{
        self.skip_whitespace();
        self.skip_comment();
        let mut t = Token::empty();

        if self.current_char == '+'{
            t = Token::new(self.current_char.to_string(), TokenType::PLUS);
        }
        else if self.current_char == '-'{
            t = Token::new(self.current_char.to_string(), TokenType::MINUS);
        }
        else if self.current_char == '*'{
            t = Token::new(self.current_char.to_string(), TokenType::ASTERISK);
        }
        else if self.current_char == '/'{
            t = Token::new(self.current_char.to_string(), TokenType::SLASH);
        }
        else if self.current_char == '='{
            // = or ==
            if self.peek() == '='{
                self.next_char();
                t = Token::new("==".to_string(), TokenType::EQEQ);
            }
            else {
                t = Token::new('='.to_string(), TokenType::EQ);
            }
        }
        else if self.current_char == '>'{
            // > or >=
            if self.peek() == '='{
                self.next_char();
                t = Token::new(">=".to_string(), TokenType::GTEQ);
            }
            else {
                t = Token::new('>'.to_string(), TokenType::GT);
            }
        }
        else if self.current_char == '<'{
            // < or <=
            if self.peek() == '='{
                self.next_char();
                t = Token::new("<=".to_string(), TokenType::LTEQ);
            }
            else {
                t = Token::new('='.to_string(), TokenType::LT);
            }

        }
        else if self.current_char == '!'{
            // Expect != , if not, error.
            if self.peek() == '='{
                self.next_char();
                t = Token::new("!=".to_string(), TokenType::NOTEQ);
            }
            else {
                self.abort(format!("Expected `!=`, found `!{}` instead.", self.peek()));
            }
        }
        else if self.current_char == '\"'{
            // Tokenize strings from the source.
            self.next_char();
            let start_pos = self.current_pos;

            while self.current_char != '\"'{
                // Let's not allow special character in a string.
                if self.current_char == '\r' || self.current_char == '\n' || self.current_char == '\t' || self.current_char == '\\' || self.current_char == '%'{
                    self.abort("illegal character in a string.".to_string());
                }

                self.next_char();
            }

            let token_text = self.source.get(start_pos as usize..self.current_pos as usize).unwrap();
            t = Token::new(String::from(token_text), TokenType::STRING);
        }
        else if self.current_char.is_ascii_digit(){
            // Tokenize the number, integer and float.
            let start_pos = self.current_pos;

            while self.peek().is_ascii_digit(){
                self.next_char();
            }

            if self.peek() == '.'{
                self.next_char();
                // The number has a decimal. Should be at least one number after the dot.
                if !self.peek().is_ascii_digit(){
                    self.abort("no decimal value specified after `.`.".to_string());
                }

                while self.peek().is_ascii_digit(){
                    self.next_char();
                }
            }

            let token_text = self.source.get(start_pos as usize..=self.current_pos as usize).unwrap();
            t = Token::new(String::from(token_text), TokenType::NUMBER);
        } 
        else if self.current_char.is_alphabetic(){
            // The token starts with an alphabetic letter. It might be a keyword or an identifier.
            // From now on, we can check if there are numbers on the token too.
            let start_pos = self.current_pos;
            while self.peek().is_alphanumeric(){
                self.next_char();
            }

            // Is the token in the list of keywords?
            let token_text = self.source.get(start_pos as usize..=self.current_pos as usize).unwrap();
            let keyword = Token::check_if_keyword(token_text);
            if matches!(keyword, TokenType::IDENT){
                t = Token::new(String::from(token_text), TokenType::IDENT);
            }
            else {
                t = Token::new(String::from(token_text), keyword);
            }

        }
        else if self.current_char == '\n'{
            self.current_line += 1;
            t = Token::new(self.current_char.to_string(), TokenType::NEWLINE);
        }
        else if self.current_char == '\0'{
            t = Token::new(self.current_char.to_string(), TokenType::EOF);
        }
        else {
            self.abort(format!("unknown token `{}`.", self.current_char));
        }

        self.next_char();
        return t;
    }
}
