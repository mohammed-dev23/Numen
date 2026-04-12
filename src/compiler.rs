use crate::scanner::{Scanner, TokenType};

pub fn compile(source: &str) {
    let mut scanner = Scanner::new_scanner(source);
    let mut line = -1 as isize;

    loop {
        let token = scanner.scan_tokens();
        if token.line != line as usize {
            println!("{:4} ", token.len);
            line = token.line as isize;
        } else {
            println!("   |  ");
        }
        println!("{:?} '{}'", token.token_type, &token.start[..token.len]);

        if token.token_type == TokenType::TEof {
            break;
        }
    }
}
