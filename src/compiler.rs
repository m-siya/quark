use crate::scanner::{Token, TokenType, Scanner};

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

        println!(" {:?}  {:?} ", token.token_type, token.lexeme);

        if token.token_type == TokenType::Eof {
            break;
        }
        
    }

}