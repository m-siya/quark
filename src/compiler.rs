use crate::{scanner::{Token, TokenType, Scanner}, chunk::{Chunk, OpCode}, value::Value, object::Object};
use std::str;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
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

impl From<u8> for Precedence {
    fn from(precedence: u8) -> Precedence {
        match precedence {
            1 => Precedence::None,
            2 => Precedence::Assignment,
            3 => Precedence::Or,
            4 => Precedence::And,
            5 => Precedence::Equality,
            6 => Precedence::Comparison,
            7 => Precedence::Term,
            8 => Precedence::Factor,
            9 => Precedence::Unary,
            10 => Precedence::Call,
            11 => Precedence::Primary,
            _ => panic!("Error. Invalid Precedence code"),
        }
    }
}

type ParseFn = fn(&mut Compiler);
#[derive(Copy, Clone)]
struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

struct Parser<'a> {
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
    panic_mode: bool,
}

impl<'a> Parser<'a> {
    pub fn new() -> Self {
        Parser { current: Token::new() , previous: Token::new(), had_error: false, panic_mode: false}
    }
}

pub struct Compiler<'a> {
    chunk: &'a mut Chunk,
    parser: Parser<'a>,
    scanner: Scanner<'a>,
    rules: Vec<ParseRule>,
}

impl <'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk, source: &'a str) -> Self {
        let mut rules = vec![
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            }; TokenType::NumberOfTokens as usize 
        ];

        rules[TokenType::LeftParen as usize] = ParseRule {
            prefix: Some(|compiler| compiler.grouping()),
            infix: None,
            precedence: Precedence::None,
        };

        rules[TokenType::RightParen as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::LeftBrace as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::RightBrace as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Comma as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Dot as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Minus as usize] = ParseRule {
            prefix: Some(|compiler| compiler.unary()),
            infix: Some(|compiler| compiler.binary()),
            precedence: Precedence::Term,
        };
    
        rules[TokenType::Plus as usize] = ParseRule {
            prefix: None,
            infix: Some(|compiler| compiler.binary()),
            precedence: Precedence::Term,
        };
    
        rules[TokenType::Semicolon as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Slash as usize] = ParseRule {
            prefix: None,
            infix: Some(|compiler| compiler.binary()),
            precedence: Precedence::Factor,
        };
    
        rules[TokenType::Star as usize] = ParseRule {
            prefix: None,
            infix: Some(|compiler| compiler.binary()),
            precedence: Precedence::Factor,
        };
    
        rules[TokenType::Bang as usize] = ParseRule {
            prefix: Some(|compiler| compiler.unary()),
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::BangEqual as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler| compiler.binary()),
            precedence: Precedence::Equality,
        };
    
        rules[TokenType::Equal as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::EqualEqual as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler| compiler.binary()),
            precedence: Precedence::Equality,
        };
    
        rules[TokenType::Greater as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler| compiler.binary()),
            precedence: Precedence::Comparison,
        };
    
        rules[TokenType::GreaterEqual as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler| compiler.binary()),
            precedence: Precedence::Comparison,
        };
    
        rules[TokenType::Less as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler| compiler.binary()),
            precedence: Precedence::Comparison,
        };
    
        rules[TokenType::LessEqual as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler| compiler.binary()),
            precedence: Precedence::Comparison,
        };
    
        rules[TokenType::Identifier as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::String as usize] = ParseRule {
            prefix: Some(|compiler| compiler.string()),
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Number as usize] = ParseRule {
            prefix: Some(|compiler| compiler.number()),
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::And as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Else as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::False as usize] = ParseRule {
            prefix: Some(|compiler| compiler.literal()),
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Function as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::If as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Void as usize] = ParseRule {
            prefix: Some(|compiler| compiler.literal()),
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Or as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Emit as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Return as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::True as usize] = ParseRule {
            prefix: Some(|compiler| compiler.literal()),
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Create as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::While as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Error as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Eof as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };

        Compiler { chunk: chunk, parser: Parser::new(), scanner: Scanner::new(source), rules: rules}


    }

    fn get_rule(&self, token_type: TokenType) -> Option<&ParseRule> {
        self.rules.get::<usize>(token_type.into())
    }

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

    fn consume(&mut self, _token_type: TokenType, message: &str) {
        match self.parser.current.token_type {
            _token_type => self.advance(),
            _ => self.error_at_current(message),
        };
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();

        let rule = self.get_rule(self.parser.previous.token_type);

        if rule.is_none() {
            eprintln!("Invalid token. Syntax Error");
            return;
        }

        let prefix_rule = rule.unwrap().prefix;

        if prefix_rule.is_none() {
            eprintln!("Expecting expression");
            return;
        }

        prefix_rule.unwrap()(self);

        while precedence <= self.get_rule(self.parser.current.token_type).unwrap().precedence {
            self.advance();
            let rule = self.get_rule(self.parser.previous.token_type);
            if rule.is_none() {
                eprintln!("Invalid token. Syntax Error");
                return;
            }
            
            let infix_rule = rule.unwrap().infix;
            infix_rule.unwrap()(self);


        }
        // self.advance();
        // if let Some(prefix_rule) = self.rules[self.parser.previous.token_type as usize].prefix {
        //     prefix_rule(self);

        //     while precedence <= self.rules[self.parser.current.token_type as usize].precedence {
        //         self.advance();
        //         if let Some(infix_rule) = self.rules[self.parser.previous.token_type as usize].infix {
        //             infix_rule(self);
        //         }
        //     }
        // } else {
        //     println!("Expect expression");
        // }
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
        let constant = self.make_constant(value);
        self.emit_bytes(OpCode::OpConstant.into(), constant);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
    }

    fn number(&mut self) {
        let value = str::from_utf8(self.parser.previous.lexeme).unwrap().parse::<f64>().unwrap();
        self.emit_constant(Value::ValNumber(value));
    }

    fn unary(&mut self) {
        let operator_type = self.parser.previous.token_type;

        self.expression();

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::OpNegate.into()),
            TokenType::Bang => self.emit_byte(OpCode::OpNot.into()),
            _ => return,
        }
    }

    fn binary(&mut self) {
        let operator_type: TokenType = self.parser.previous.token_type;
        let rule: Option<&ParseRule> = self.get_rule(operator_type);

        if rule.is_none() {
            eprintln!("Invalid token type and precedence not found in precedence rules");
            return;
        }

        self.parse_precedence(Precedence::from(u8::from(rule.unwrap().precedence) + 1));
        

        match operator_type {
            TokenType::BangEqual => self.emit_bytes(OpCode::OpEqual.into(), OpCode::OpNot.into()),
            TokenType::EqualEqual => self.emit_byte(OpCode::OpEqual.into()),
            TokenType::Greater => self.emit_byte(OpCode::OpGreater.into()),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::OpLess.into(), OpCode::OpNot.into()),
            TokenType::Less => self.emit_byte(OpCode::OpLess.into()),
            TokenType::LessEqual => self.emit_bytes(OpCode::OpGreater.into(), OpCode::OpNot.into()),
            TokenType::Plus => {
                self.emit_byte(OpCode::OpAdd.into());
            },
            TokenType::Minus => {
                self.emit_byte(OpCode::OpSubtract.into());
            },
            TokenType::Star => {
                self.emit_byte(OpCode::OpMultiply.into());
            },
            TokenType::Slash => {
                self.emit_byte(OpCode::OpDivide.into());
            },

            _ => {
                return;
            }

        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "Expecting ')' after expression.");

    }

    fn literal(&mut self) {
        match self.parser.previous.token_type {
            TokenType::False => self.emit_byte(OpCode::OpFalse.into()),
            TokenType::Void => self.emit_byte(OpCode::OpVoid.into()),
            TokenType::True => self.emit_byte(OpCode::OpTrue.into()),
            _ => {
                return;
            }
        }
    }

    fn string(&mut self) {
        self.emit_constant(Value::ValObject(
            Object::from_str(
                str::from_utf8(
                    self.parser.previous.lexeme).unwrap_or("")
                    .trim_start_matches('"').trim_end_matches('"'))));
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.parser.previous, message);

    }

    fn error_at(&mut self, token: Token, message: &str) {
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

    pub fn compile(&mut self) -> bool {
        //let mut scanner = Scanner::new(source);
    
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
        self.end_compiler();

        !self.parser.had_error


    }
}
