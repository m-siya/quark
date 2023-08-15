use crate::scanner::{Token, TokenType, Scanner};
use std::str;

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);

    let mut line = -1;

    loop {
        let token: Token = scanner.scan_token();
        if token.line != line {
            print!("{}  ", token.line);
            line = token.line;
        } else {
            print!("   | ");
        }

        println!(" {:?}  {:?} ", token.token_type, str::from_utf8(token.lexeme).unwrap_or("Character not supported in utf-8"));

        if token.token_type == TokenType::Eof {
            // println!("hi! i end here");
            break;
        }
        
    }

}