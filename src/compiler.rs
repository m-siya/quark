use crate::{scanner::{Token, TokenType, Scanner}, chunk::{Chunk, OpCode}, value::Value};
use std::str;

enum Precedence {
    None,
    Assignment, 
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary
}

impl From<Precedence> for u8 {
    fn from(precedence: Precedence) -> u8 {
        precedence as u8
    }
}

struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
}

pub struct Compiler<'a> {
    chunk: &'a Chunk,
    parser: Parser<'a>,
    scanner: Scanner<'a>
}

impl Compiler<'_> {
    fn advance(&mut self) {
        self.parser.previous = self.parser.current;
        
        loop {
            self.parser.current = self.scanner.scan_token();
            if self.parser.current.token_type != TokenType::Error {
                break;
            }

            self.error_at_current(str::from_utf8(self.parser.current.lexeme).unwrap());
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        match self.parser.current.token_type {
            token_type => self.advance(),
            _ => self.error_at_current(message),
        };
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&self, precedence: Precedence) {

    }

    // append a single byte to the chunk
    fn emit_byte(&mut self, byte: u8){
        self.chunk.write(byte, self.parser.previous.line);
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OpReturn.into());
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        let constant = self.chunk.add_constant(value);

        if constant > 256 {
            eprint!("Too many constants in one chunk");
            return 0;
        }
        constant as u8     
    }

    fn emit_constant(&mut self, value: Value) {
        self.emit_bytes(OpCode::OpConstant.into(), self.make_constant(value));
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn number(&self) {
        let value = str::from_utf8(self.parser.previous.lexeme).unwrap().parse::<f64>().unwrap();
        self.emit_constant(Value::ValNumber(value));
    }

    fn unary(&self) {
        let operator_type = self.parser.previous.token_type;

        self.expression();

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate.into()),
            _ => return,
        }
    }

    // fn binary(&self) {
    //     let operator_type: TokenType = self.parser.previous.token_type;
    //     let rule: ParseRule = get_rule(operator_type);
        
    // }

    fn grouping(&self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expecting ')' after expression.");

    }

    fn error_at_current(&self, message: &str) {
        self.error_at(&self.parser.previous, message);

    }

    fn error_at(&self, token: &Token, message: &str) {
        if self.parser.panic_mode {
            return
        }

        self.parser.panic_mode = true;

        eprint!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Eof => {
                eprint!(" at the end");
            },
            TokenType::Error => {},
            _ => {
                eprint!(" at  {}", str::from_utf8(token.lexeme).unwrap());
            }
        }

        eprint!(" : {}", message);

        self.parser.had_error = true;


    }

    pub fn compile(&mut self, source: &str) -> bool {
        let mut scanner = Scanner::new(source);
    
        // let mut line = -1;
    
        // loop {
        //     let token: Token = scanner.scan_token();
        //     if token.line != line {
        //         print!("{}  ", token.line);
        //         line = token.line;
        //     } else {
        //         print!("   | ");
        //     }
    
        //     println!(" {:?}  {:?} ", token.token_type, str::from_utf8(token.lexeme).unwrap_or("Character not supported in utf-8"));
    
        //     if token.token_type == TokenType::Eof {
        //         // println!("hi! i end here");
        //         break;
        //     }
            
        // }
    
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expecting end of expression");

        !self.parser.had_error


    }
}
