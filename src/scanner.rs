use std::collections::HashMap;

#[derive(Debug)]
pub struct Scanner<'s> {
    pub start: &'s str,
    pub current: &'s str,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: String,
    pub len: usize,
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(usize)]
pub enum TokenType {
    TLr,
    TRr,
    TPlus,
    Tminus,
    TSemicolon,
    Tint,
    Tfloat,
    TStr,
    TId,
    TEof,
    TErr,
    TdivOp,
    TmulOp,
    TmodOp,
    TpowOp,
    TdivdivOp,
    Ttrue,
    Tfalse,
    Tnot,
    Teqeq,
    Teq,
    TnotEq,
    Tgt,
    Tlt,
    Tgte,
    Tlte,
    Tprint,
    Tmake,
    Tlb,
    Trb,
}

impl<'s> Scanner<'s> {
    pub fn new_scanner(source: &'s str) -> Self {
        Scanner {
            start: source,
            current: source,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Token {
        self.unconsumable();
        self.start = self.current;

        if self.is_at_end() {
            return self.generate_token(TokenType::TEof);
        }

        let c = self.advance();
        match c {
            '+' => self.generate_token(TokenType::TPlus),
            ';' => self.generate_token(TokenType::TSemicolon),
            '"' => self.string_tokens(),
            c if c.is_ascii_digit() => self.num_tokens(),
            c if c.is_ascii_alphanumeric() => self.identifier(),
            '(' => self.generate_token(TokenType::TLr),
            ')' => self.generate_token(TokenType::TRr),
            '-' => self.generate_token(TokenType::Tminus),
            '%' => self.generate_token(TokenType::TmodOp),
            '*' => self.generate_token(TokenType::TmulOp),
            '^' => self.generate_token(TokenType::TpowOp),
            '/' => {
                if self.match_tokens('/') {
                    self.generate_token(TokenType::TdivdivOp)
                } else {
                    self.generate_token(TokenType::TdivOp)
                }
            }
            '=' => {
                if self.match_tokens('=') {
                    self.generate_token(TokenType::Teqeq)
                } else {
                    self.generate_token(TokenType::Teq)
                }
            }
            '!' => {
                if self.match_tokens('=') {
                    self.generate_token(TokenType::TnotEq)
                } else {
                    self.generate_token(TokenType::Tnot)
                }
            }
            '>' => {
                if self.match_tokens('=') {
                    self.generate_token(TokenType::Tgte)
                } else {
                    self.generate_token(TokenType::Tgt)
                }
            }
            '<' => {
                if self.match_tokens('=') {
                    self.generate_token(TokenType::Tlte)
                } else {
                    self.generate_token(TokenType::Tlt)
                }
            }
            '}' => self.generate_token(TokenType::Trb),
            '{' => self.generate_token(TokenType::Tlb),
            _ => self.err_token("Unexpected character."),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current.is_empty()
    }

    fn generate_token(&self, t_type: TokenType) -> Token {
        let len = self.start.len() - self.current.len();

        Token {
            token_type: t_type,
            start: self.start[..len].to_string(),
            len,
            line: self.line,
        }
    }

    fn err_token(&self, message: &'s str) -> Token {
        Token {
            token_type: TokenType::TErr,
            start: message.to_string(),
            len: message.len(),
            line: self.line,
        }
    }

    fn advance(&mut self) -> char {
        let c = self.current.chars().next().unwrap();
        self.current = &self.current[1..];
        c
    }

    fn peek(&self) -> char {
        self.current.chars().next().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.current[1..].chars().next().unwrap_or('\0')
    }

    fn match_tokens(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let char: Vec<char> = self.current.chars().collect();

        if char[0] != expected {
            return false;
        }

        self.current = &self.current[1..];

        true
    }

    fn unconsumable(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    continue;
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    continue;
                }
                '#' => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn string_tokens(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.err_token("Unterminated string.");
        }

        self.advance();
        self.generate_token(TokenType::TStr)
    }

    fn num_tokens(&mut self) -> Token {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
            return self.generate_token(TokenType::Tfloat);
        }
        self.generate_token(TokenType::Tint)
    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_ascii_alphabetic() || self.peek().is_ascii_digit() {
            self.advance();
        }

        let text = &self.start[..self.start.len() - self.current.len()];

        let keywords: HashMap<&str, TokenType> = HashMap::from([
            ("true", TokenType::Ttrue),
            ("false", TokenType::Tfalse),
            ("print", TokenType::Tprint),
            ("make", TokenType::Tmake),
        ]);

        let t_type = keywords.get(text).unwrap_or(&TokenType::TId);
        self.generate_token(match *t_type {
            TokenType::Ttrue => TokenType::Ttrue,
            TokenType::Tfalse => TokenType::Tfalse,
            TokenType::Tprint => TokenType::Tprint,
            TokenType::Tmake => TokenType::Tmake,
            _ => TokenType::TId,
        })
    }
}
