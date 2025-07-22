use crate::{scanner::{Token, TokenType, Scanner}, chunk::{Chunk, OpCode}, value::Value, object::{Object, ObjString}};
use std::str;

#[cfg(feature = "trace")]
use trace::trace;
#[cfg(feature = "trace")]
trace::init_depth_var!();

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
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

type ParseFn = fn(&mut Compiler, can_assign: bool);
#[derive(Copy, Clone, Debug)]
struct ParseRule {
    prefix: Option<ParseFn>,
    infix: Option<ParseFn>,
    precedence: Precedence,
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Local<'a> {
    name: Token<'a>,
    depth: i32,
}

impl <'a> Local <'a> {
    pub fn new(name: Token<'a>, depth: i32) -> Self {
        Local { name: name, depth: depth }
    }
}

#[derive(Debug)]
struct Scope <'a>{
    locals: Vec<Local<'a>>,
    scope_depth: i32, // numberof blocks surrounding the current bit of code we're compiling
}

impl <'a> Scope <'a>{
    pub fn new() -> Self {
        Self { locals: Vec::new(), scope_depth: 0}
    }    


    pub fn add_local(&mut self, name: Token<'a>) {
        let local: Local = Local::new(name, -1);
        self.locals.push(local);
    }
}

#[derive(Debug)]
pub struct Compiler<'a> {
    chunk: &'a mut Chunk,
    parser: Parser<'a>,
    scanner: Scanner<'a>,
    rules: Vec<ParseRule>,
    scope: Scope<'a>,
}

#[cfg_attr(feature = "trace", trace)]

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
            prefix: Some(|compiler, _can_assign| compiler.grouping()),
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
            prefix: Some(|compiler, _can_assign| compiler.unary()),
            infix: Some(|compiler, _can_assign| compiler.binary()),
            precedence: Precedence::Term,
        };
    
        rules[TokenType::Plus as usize] = ParseRule {
            prefix: None,
            infix: Some(|compiler, _can_assign| compiler.binary()),
            precedence: Precedence::Term,
        };
    
        rules[TokenType::Semicolon as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Slash as usize] = ParseRule {
            prefix: None,
            infix: Some(|compiler, _can_assign| compiler.binary()),
            precedence: Precedence::Factor,
        };
    
        rules[TokenType::Star as usize] = ParseRule {
            prefix: None,
            infix: Some(|compiler, _can_assign| compiler.binary()),
            precedence: Precedence::Factor,
        };
    
        rules[TokenType::Bang as usize] = ParseRule {
            prefix: Some(|compiler, _can_assign| compiler.unary()),
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::BangEqual as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler, _can_assign| compiler.binary()),
            precedence: Precedence::Equality,
        };
    
        rules[TokenType::Equal as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::EqualEqual as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler, _can_assign| compiler.binary()),
            precedence: Precedence::Equality,
        };
    
        rules[TokenType::Greater as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler, _can_assign| compiler.binary()),
            precedence: Precedence::Comparison,
        };
    
        rules[TokenType::GreaterEqual as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler, _can_assign| compiler.binary()),
            precedence: Precedence::Comparison,
        };
    
        rules[TokenType::Less as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler, _can_assign| compiler.binary()),
            precedence: Precedence::Comparison,
        };
    
        rules[TokenType::LessEqual as usize] = ParseRule {
            prefix: None,
            infix:  Some(|compiler, _can_assign| compiler.binary()),
            precedence: Precedence::Comparison,
        };
    
        rules[TokenType::Identifier as usize] = ParseRule {
            prefix: Some(|compiler, can_assign| compiler.variable(can_assign)),
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::String as usize] = ParseRule {
            prefix: Some(|compiler, _can_assign| compiler.string()),
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Number as usize] = ParseRule {
            prefix: Some(|compiler, _can_assign| compiler.number()),
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::And as usize] = ParseRule {
            prefix: None,
            infix: Some(|compiler, can_assign| compiler.my_and(can_assign)),
            precedence: Precedence::And,
        };
    
        rules[TokenType::Else as usize] = ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::False as usize] = ParseRule {
            prefix: Some(|compiler, _can_assign| compiler.literal()),
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
            prefix: Some(|compiler, _can_assign| compiler.literal()),
            infix: None,
            precedence: Precedence::None,
        };
    
        rules[TokenType::Or as usize] = ParseRule {
            prefix: None,
            infix: Some(|compiler, can_assign| compiler.my_or(can_assign)),
            precedence: Precedence::Or,
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
            prefix: Some(|compiler, _can_assign| compiler.literal()),
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

        Compiler { chunk: chunk, parser: Parser::new(), scanner: Scanner::new(source), rules: rules, scope: Scope::new()}
    }

    fn get_rule(&self, token_type: TokenType) -> Option<&ParseRule> {
        self.rules.get::<usize>(token_type.into())
    }
  
    fn begin_scope(&mut self) {
        self.scope.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.scope.scope_depth -= 1;

        while let Some(last) = self.scope.locals.last() {
            if last.depth > self.scope.scope_depth {
                self.emit_byte(OpCode::OpPop.into());
                self.scope.locals.pop();
            }
        }
    }

    /*
        store current token in previous token. 
        Scan the next token and if encounter an Error token, break and report error. 
    */
    fn advance(&mut self) {
        self.parser.previous = self.parser.current;
        
        loop {
            // scan the next token (scan more source code until find a valid lexeme and convert to token) and store in current
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
            //_ => self.error_at_current(message),
        };
    }

    fn is_match(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }

        self.advance();
        true
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.parser.current.token_type == token_type
    }

    fn my_and(&mut self, _can_assign: bool) {
        let end_jump = self.emit_jump(OpCode::OpJumpIfFalse.into());

        self.emit_byte(OpCode::OpPop.into());
        self.parse_precedence(Precedence::And);

        self.patch_jump(end_jump);
    }

    fn my_or(&mut self, _can_assign: bool) {
        let else_jump = self.emit_jump(OpCode::OpJumpIfFalse.into());
        let end_jump = self.emit_jump(OpCode::OpJump.into());

        self.patch_jump(else_jump);
        self.emit_byte(OpCode::OpPop.into());

        self.parse_precedence(Precedence::Or);
        self.patch_jump(end_jump);
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn var_declaration(&mut self) {
        let global: u8 = self.parse_variable("Expecting variable name.");

        if self.is_match(TokenType::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::OpVoid.into());
        }

        self.consume(TokenType::Semicolon, "Expecting ';' after expression");

        self.define_variable(global);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expecting ';' after expression");
        self.emit_byte(OpCode::OpPop.into());
    }

    fn emit_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expecting ';' after value.");
        self.emit_byte(OpCode::OpEmit.into());
    }

    fn declaration(&mut self) {
        //self.statement();
        if self.is_match(TokenType::Create) {
            self.var_declaration();
        } else {
            self.statement();
        }

        if self.parser.panic_mode {
            self.synchronize();
        }
    }

    fn synchronize(&mut self) {
        //hit compile error while parsing previous statement. start synchronizing

        self.parser.panic_mode = false;

        while self.parser.current.token_type != TokenType::Eof {
            if self.parser.previous.token_type == TokenType::Semicolon {
                return;
            }

            match self.parser.current.token_type {
                TokenType::Function | TokenType::Create | TokenType::While | TokenType::Emit | TokenType::Return => {
                    return;
                },
                _ => (),
            }

            self.advance();
        }
    }

    fn if_statement(&mut self) {
        self.consume(TokenType::LeftParen, "Expecting '(' after 'if'.");
        self.expression();
        self.consume(TokenType::RightParen, "Expecting ')' after condition.");

        let then_jump = self.emit_jump(OpCode::OpJumpIfFalse.into()); //opcode has operand for how much to offset the ip
        self.emit_byte(OpCode::OpPop.into());

        self.statement();
        let else_jump = self.emit_jump(OpCode::OpJump.into());

        self.patch_jump(then_jump);
        self.emit_byte(OpCode::OpPop.into());


        if self.is_match(TokenType::Else) {
            self.statement();
        }

        self.patch_jump(else_jump); // is unconditional
    }

    fn emit_jump(&mut self, instruction: u8) -> usize {
        self.emit_byte(instruction);
        //placeholder operands
        self.emit_byte(0xff);
        self.emit_byte(0xff);
        //return the place where we are rn (barring the placeholder operands)
        self.chunk.code.len() - 2
    }

    fn patch_jump(&mut self, offset: usize) {
        let jump = self.chunk.code.len() - offset - 2;

        if jump as u16 > u16::MAX {
            self.error_at_current("Too much code to jump over");
        }
        //println!("chunk before patch jump {:?}, index - {}", self.chunk.code, offset);
        //get high byte of the two bytes set aside for jump
        // jump >> 8 will isolate the high byte and & 0xff will ensure only 8 lsb are retained
        if let Some(bytecode_offset) = self.chunk.code.get_mut(offset) {
            *bytecode_offset = ((jump >> 8) & 0xff) as u8;
        }
        //get low byte
        if let Some(bytecode_offset) = self.chunk.code.get_mut(offset + 1) {
            *bytecode_offset = (jump & 0xff) as u8;
        }

       // println!("chunk after patch jump {:?}", self.chunk.code);

        

    }

    fn while_statement(&mut self) {
        let loop_start = self.chunk.code.len(); //capture location of start of loop
        self.consume(TokenType::LeftParen, "Expecting '(' after 'while'.");
        self.expression();
        self.consume(TokenType::While, "Expecting ')' after condition.");

        let exit_jump = self.emit_jump(OpCode::OpJumpIfFalse.into());
        self.emit_byte(OpCode::OpPop.into());
        self.statement();
        self.emit_loop(loop_start); //to jump backward

        self.patch_jump(exit_jump);
        self.emit_byte(OpCode::OpPop.into());
    }



    fn statement(&mut self) {
        if self.is_match(TokenType::Emit) {
            self.emit_statement();
        } else if self.is_match(TokenType::If) {
            self.if_statement();
        } else if self.is_match(TokenType::While){
            self.while_statement();
        } else if self.is_match(TokenType::LeftBrace){
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
            self.expression_statement();
        }
    }

    fn block(&mut self) {
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            self.declaration();
        }

        self.consume(TokenType::RightBrace, "Expecting '}' after block");
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

       // prefix_rule.unwrap()(self);
       let can_assign: bool = precedence <= Precedence::Assignment;
       prefix_rule.unwrap()(self, can_assign);

        while precedence <= self.get_rule(self.parser.current.token_type).unwrap().precedence {
            self.advance();
            let rule = self.get_rule(self.parser.previous.token_type);
            if rule.is_none() {
                eprintln!("Invalid token. Syntax Error");
                return;
            }
            
            let infix_rule = rule.unwrap().infix;
            infix_rule.unwrap()(self, can_assign);
        }
        
        
        if can_assign && self.is_match(TokenType::Equal) {
            self.error_at(self.parser.previous, "Invalid assignment target");
        }

    }

    fn identifier_constant(&mut self, name: Token) -> u8 {
        // global variables are looked up by name at runtime. so vm needs access to name. cannot put
        // whole string into bytecode so put in chunk's constant array and refer by index.
        self.make_constant(
            Value::ValObject(
                Object::ObjString(
                    ObjString::from_str(
                        str::from_utf8(name.lexeme).unwrap()
                    )
                )
            )
        )
    }

    fn parse_variable(&mut self, error_message: &str) -> u8 {
        self.consume(TokenType::Identifier, error_message);

        self.declare_variable();

        if self.scope.scope_depth > 0 {  // if scope is not global
            return 0;
        }
        self.identifier_constant(self.parser.previous)
    }

    fn declare_variable(&mut self) {
        if self.scope.scope_depth == 0 {
            return;
        }

        let name: Token = self.parser.previous;

        for local in self.scope.locals.iter().rev().take_while(|local| local.depth == -1 || local.depth >= self.scope.scope_depth) {
            if name.lexeme == local.name.lexeme {
                self.error_at_current("Already a variable with this name in this scope");
                return;
            }
        }


        self.scope.add_local(name);
    }

    fn define_variable(&mut self, global: u8) {
        if self.scope.scope_depth > 0 {
            self.mark_initialized();
            return;
        }
        self.emit_bytes(OpCode::OpDefineGlobal.into(), global);
    }

    fn mark_initialized(&mut self) {
        self.scope.locals.last_mut().unwrap().depth = self.scope.scope_depth;
    }

    /*
        write Opcode in u8 form to chunk along with line number
    */
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

    fn emit_loop(&mut self, loop_start: usize) {
        self.emit_byte(OpCode::OpLoop.into());

        let offset = self.chunk.code.len() - loop_start + 2;
        if offset as u16 > u16::MAX {
            self.error_at_current("Loop body too large");
        } 

        self.emit_byte(((offset >> 8) & 0xff) as u8);
        self.emit_byte((offset & 0xff) as u8);


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
        // self.parser.previous.lexeme is a byte array. 
        // convert it to a string. 
        // then parse it to a f64.
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
        self.emit_constant(
            Value::ValObject(
                Object::ObjString(
                    ObjString::from_str(
                        str::from_utf8(
                            self.parser.previous.lexeme)
                            .unwrap_or("")
                            .trim_start_matches('"').trim_end_matches('"')
                        )
                    )
                )
            );
            

        // self.emit_constant(Value::ValObject(
        //     Object::from_str(
                // str::from_utf8(
                //     self.parser.previous.lexeme).unwrap_or("")
                //     .trim_start_matches('"').trim_end_matches('"'))));
    }

    fn named_variable(&mut self, name: Token, can_assign: bool) {
        let (arg, set_op, get_op) = if let Some(index) = self.resolve_local(name) {
            (index as u8, OpCode::OpSetLocal, OpCode::OpGetLocal)
        } else {
            (self.identifier_constant(name), OpCode::OpSetGlobal, OpCode::OpGetGlobal)
        };

        //println!("{} {:?} {:?}", arg, set_op, get_op);

        if can_assign && self.is_match(TokenType::Equal) {
            self.expression();
            self.emit_bytes(set_op.into(), arg);
        } else {
            self.emit_bytes(get_op.into(), arg)
        }

    }

    fn resolve_local(&mut self, name: Token) -> Option<usize> {
        for (index, local)  in self.scope.locals.iter().enumerate().rev() {
            if name.lexeme == local.name.lexeme {
                if local.depth == -1 {
                    self.error_at_current("Cannot read local variable in its own initializer.");
                }
                return Some(index)
            }
        }
        None
    }
    
    fn variable(&mut self, can_assign: bool) {
        self.named_variable(self.parser.previous, can_assign);
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

    /*
        Compile the source code into bytecode and store it in the chunk.
    */
    pub fn compile(&mut self) -> bool {        
        self.advance();

        while !self.is_match(TokenType::Eof) {
            self.declaration();
        }

        self.end_compiler();

        !self.parser.had_error

    }

}
