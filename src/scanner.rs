
use std::str;

#[derive(PartialEq, Debug, Clone, Copy)]
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
    While, For,

    Error, Eof,

    NumberOfTokens,
}

impl From<TokenType> for usize {
    fn from(token_type: TokenType)  -> usize {
        token_type as usize
    }
}

/*
    Token - smallest unit of source code that is meaningful to compiler

    Fields:
    - token_type: type of the token (e.g. identifier, number, string, etc.)
    - lexeme: the actual text of the token, is a reference to a slice of the source code
    - line: the line number in the source code where the token was found
*/
#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    // start: usize,
    // length: i32,
    pub lexeme: &'a [u8],
    pub line: i32,
}

impl <'a> Token <'a> {
    pub fn new() -> Self {
        Token { token_type: TokenType::Eof, lexeme: b"", line: -1}
    }

    fn make_token(token_type: TokenType, lexeme: &[u8], line: i32) -> Token{
        Token {token_type, lexeme, line}
    }

    fn make_error_token(message: &str, line: i32) -> Token {
        Token {token_type: TokenType::Error, lexeme: message.as_bytes(), line: line}
    }

    pub fn to_lexeme(&self) -> &str {
        str::from_utf8(self.lexeme.clone()).unwrap()
    }
}

#[derive(Debug)]
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

    /*
      Returns true if the scanner has reached the end of the source code.
    */
    fn is_at_end(&self) -> bool{
        return self.current == self.source.len()
    }

    /*
    
        Advance scanner to next character in source code.
        Returns the character that was advanced to, or None if at end of source code.
    */
    fn advance(&mut self) -> Option<&u8>{
        
        self.current += 1;
        // print!(" {:?} ", str::from_utf8(&self.source[self.start..self.current]).unwrap());

        self.source.get(self.current - 1)
    }

    fn match_(&mut self, _expected: u8) -> bool {
        match self.source.get(self.current) {
            None => false,
            Some(c) => {
                if c == &_expected {
                    self.current += 1;
                    true
                } else {
                    false
                }
            },
        }

        // match self.source.get(self.current) {
        //     Some(actual) if actual == &_expected => {
        //         self.current += 1;
        //         true
        //     }
        //     _ => false,
        // }
    }

    fn peek(&self) -> Option<&u8> {
        self.source.get(self.current)
    }

    fn peek_next(&self) -> Option<&u8> {
        self.source.get(self.current + 1)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(b' ' | b'\t' | b'\r') => {
                    self.advance();
                },
                Some(b'\n') => {
                    self.advance();
                    self.line += 1;
                },
                _ => break,
            }
        }
    }

    fn string(&mut self) -> Token<'a>{
        while self.peek() != Some(&b'"') && !self.is_at_end() {
            if self.peek() == Some(&b'\n') {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            Token::make_error_token("Unterminated String", self.line)
        } else {
            self.advance();
            Token::make_token(TokenType::String, &self.source[self.start..self.current], self.line)
        }
    }

    fn number(&mut self) -> Token<'a> {
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

    fn identifier(&mut self) -> Token<'a> {
        while self.peek().is_some() && (self.peek().unwrap().is_ascii_alphanumeric() || self.peek().unwrap() == &b'_') {
            self.advance();
        }

        Token::make_token(self.identifier_type(), &self.source[self.start..self.current], self.line)
    }
    
    /*
        check_keyword checks if the current lexeme matches a keyword and returns the corresponding TokenType.
        If it does not match, it returns TokenType::Identifier.

        fields:
        - start: the index in the lexeme where the keyword starts
        - rest: the rest of the keyword to check
        - token_type: the TokenType to return if the keyword matches
    */
    fn check_keyword(&self, start: usize, rest: &str, token_type: TokenType) -> TokenType {
        // check if the rest of the keyword matches
        if &self.source[self.start + start..self.current] == rest.as_bytes() {
            return token_type;
        }

        TokenType::Identifier
    }

    /* 
        identifier_type determines the type of identifier based on the first character
        and checks for keywords in the source code.
        
        Returns TokenType::Identifier if no keyword is found.
    */
    fn identifier_type(&self) -> TokenType {
        match self.source[self.start] {
            b'a' => self.check_keyword(1, "nd", TokenType::And),
            b'c' => self.check_keyword(1, "reate", TokenType::Create),
            b'e' => {
                match self.source.get(self.start + 1) {
                    Some(b'l') => self.check_keyword(2, "se", TokenType::Else),
                    Some(b'm') => self.check_keyword(2, "it", TokenType::Emit),
                    _ => TokenType::Identifier,
                }
            },
            b'f' => {
                match self.source.get(self.start + 1) {
                    Some(b'a') => self.check_keyword(2, "lse", TokenType::False),
                    Some(b'u') => self.check_keyword(2, "nction", TokenType::Function),
                    Some(b'o') => self.check_keyword(2, "r", TokenType::For),
                    _ => TokenType::Identifier,
                }
            },
            b'i' => self.check_keyword(1, "f", TokenType::If),
            b'o' => self.check_keyword(1, "r", TokenType::Or),
            b'r' => self.check_keyword(1, "reate", TokenType::Return),
            b't' => self.check_keyword(1, "rue", TokenType::True),
            b'v' => self.check_keyword(1, "oid", TokenType::Void),
            b'w' => self.check_keyword(1, "hile", TokenType::While),
            _ => TokenType::Identifier,

        }
    }

    /*
        Scans a single token 
    */
    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return Token::make_token(TokenType::Eof, &self.source[self.start..self.current], self.line);
        }

        let token_type = match self.advance() {
            None => TokenType::Eof,
            Some(c) =>  match c {
                b'(' => TokenType::LeftParen,
                b')' => TokenType::RightParen,
                b'{' => TokenType::LeftBrace,
                b'}' => TokenType::RightBrace,
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