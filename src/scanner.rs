#[derive(PartialEq, Debug)]
pub enum TokenType {
    //single-character tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus,
    Semicolon, Slash, Star,

    //one or two character tokens
    Bang, BangEqual, 
    Equal, EqualEqual, 
    Greater, GreaterEqual, 
    Less, LessEqual, 

    //literals
    Identifier, String, Number,

    //keywords
    And, Else, False, 
    Function, If, Void, Or,
    Emit, Return, True, Create, 
    While,

    Error, Eof
}

pub struct Token<'a> {
    pub token_type: TokenType,
    // start: usize,
    // length: i32,
    pub lexeme: &'a [u8],
    pub line: i32,
}

impl <'a> Token <'a> {
    fn make_token(token_type: TokenType, lexeme: &[u8], line: i32) -> Token{
        Token {token_type, lexeme, line}
    }

    fn make_error_token(message: &str, line: i32) -> Token {
        Token {token_type: TokenType::Error, lexeme: message.as_bytes(), line: line}
    }
}

pub struct Scanner <'a> {
    source: &'a [u8],
    start: usize, // start of current lexeme
    current: usize, // current character of current lexeme
    line: i32,
}

impl<'a> Scanner <'a> {
    pub fn new(source: &str) -> Scanner {
        Scanner{source: source.as_bytes(), start: 0, current: 0, line: 1}
    }

    fn is_at_end(&self) -> bool{
        return self.current == self.source.len()
    }

    fn advance(&mut self) -> Option<&u8>{
        self.current += 1;
        self.source.get(self.current - 1)
    }

    fn match_(&mut self, expected: u8) -> bool {

        match self.source.get(self.current) {
            None => false,
            Some(c) => match c {
                expected => {
                    self.current += 1;
                    true
                },
                _ => false
            },
        }
    }

    fn peek(&self) -> Option<&u8> {
        self.source.get(self.current)
    }

    fn peek_next(&self) -> Option<&u8> {
        self.source.get(self.current + 1)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.source.get(self.current) {
                Some(c) => match c{
                    b' ' | b'\r' | b'\t' => {
                        self.advance();
                    },
                    b'\n' => {
                        self.line += 1;
                        self.advance();
                    }
                    _ => break,
                },
                _ => break
            }
        }
    }

    fn string(&mut self) -> Token {
        while self.peek() != Some(&b'"') && (self.advance().is_some()) {
            if self.peek() == Some(&b'\n') {
                self.line += 1;
            }

            self.advance();
        }

        if self.advance().is_none() {
            Token::make_error_token("Unterminated String", self.line)
        } else {
            self.advance();
            Token::make_token(TokenType::String, &self.source[self.start..self.current], self.line)
        }
    }

    fn number(&mut self) -> Token {
        while self.peek().is_some() && self.peek().unwrap().is_ascii_digit() {
            self.advance();
        }

        if self.peek().is_some() && self.peek().unwrap() == &b'.' && self.peek_next().is_some() && self.peek_next().unwrap().is_ascii_digit() {
            self.advance();

            while self.peek().is_some() && self.peek().unwrap().is_ascii_digit() {
                self.advance();
            }
        }

        Token::make_token(TokenType::Number, &self.source[self.start..self.current], self.line)
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_some() && (self.peek().unwrap().is_ascii_alphanumeric() || self.peek().unwrap() == &b'_') {
            self.advance();
        }

        Token::make_token(self.identifier_type(), &self.source[self.start..self.current], self.line)
    }

    fn check_keyword(&self, start: usize, length: i32, rest: &str, token_type: TokenType) -> TokenType {
        if &self.source[self.start + start..self.current] == rest.as_bytes() {
            return token_type;
        }

        TokenType::Identifier
    }

    fn identifier_type(&self) -> TokenType {
        match self.source[self.start] {
            b'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            b'c' => self.check_keyword(1, 5, "reate", TokenType::Create),
            b'e' => {
                match self.source.get(self.start + 1) {
                    Some(b'l') => self.check_keyword(2, 2, "se", TokenType::Else),
                    Some(b'm') => self.check_keyword(2, 2, "it", TokenType::Emit),
                    _ => TokenType::Identifier,
                }
            },
            b'f' => {
                match self.source.get(self.start + 1) {
                    Some(b'a') => self.check_keyword(2, 3, "lse", TokenType::False),
                    Some(b'u') => self.check_keyword(2, 6, "nction", TokenType::Function),
                    _ => TokenType::Identifier,
                }
            },
            b'i' => self.check_keyword(1, 1, "f", TokenType::If),
            b'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            b'r' => self.check_keyword(1, 5, "reate", TokenType::Return),
            b't' => self.check_keyword(1, 3, "rue", TokenType::True),
            b'v' => self.check_keyword(1, 3, "oid", TokenType::Void),
            b'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,

        }
    }



    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;

        // if self.is_at_end() {
        //     return Token::make_token(TokenType::Eof, &self.source[self.start..self.current], self.line);
        // }



        let token_type = match self.advance() {
            None => TokenType::Eof,
            Some(c) =>  match c {
                b'(' => TokenType::LeftParen,
                b')' => TokenType::RightParen,
                b'{' => TokenType::LeftBrace,
                b'}' => TokenType::LeftBrace,
                b';' => TokenType::Semicolon,
                b',' => TokenType::Comma,
                b'.' => TokenType::Dot,
                b'-' => TokenType::Minus,
                b'+' => TokenType::Plus,
                b'/' => TokenType::Slash,
                b'*' => TokenType::Star,
                b'!' => { 
                    if self.match_(b'=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                },
                b'=' => { 
                    if self.match_(b'=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                },
                b'<' => { 
                    if self.match_(b'=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                },
                b'>' => { 
                    if self.match_(b'=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                },  
                b'"' => {
                    return self.string();
                }  
                c if c.is_ascii_digit() => {
                    return self.number();
                }
                c if c.is_ascii_alphanumeric() || c == &b'_' => {
                    return self.identifier();
                },
                _ => return Token::make_error_token("Unexpected character.", self.line),
            },
        };

        Token::make_token(token_type, &self.source[self.start..self.current], self.line)

        

    }

}