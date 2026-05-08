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
    Tlp,
    Trp,
    Tplus,
    Tminus,
    Tsemicolon,
    Tint,
    Tfloat,
    Tstr,
    Tid,
    Teof,
    Terr,
    TdivOp,
    TmulOp,
    TmodOp,
    TpowOp,
    TdivdivOp,
    Ttrue,
    Tfalse,
    Tnot,
    TeqEq,
    Teq,
    TnotEq,
    Tgt,
    Tlt,
    Tgte,
    Tlte,
    Tprint,
    Tset,
    Tlb,
    Trb,
    Tif,
    Telse,
    Tfix,
    Tand,
    Tor,
    Twhile,
    Tloop,
    Tstop,
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
            return self.generate_token(TokenType::Teof);
        }

        let c = self.advance();
        match c {
            '+' => self.generate_token(TokenType::Tplus),
            ';' => self.generate_token(TokenType::Tsemicolon),
            '"' => self.string_tokens(),
            c if c.is_ascii_digit() => self.num_tokens(),
            c if c.is_ascii_alphanumeric() || c == '_' => self.identifier(),
            '(' => self.generate_token(TokenType::Tlp),
            ')' => self.generate_token(TokenType::Trp),
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
                    self.generate_token(TokenType::TeqEq)
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
            token_type: TokenType::Terr,
            start: message.to_string(),
            len: message.len(),
            line: self.line,
        }
    }

    fn advance(&mut self) -> char {
        let c = self.current.chars().next().unwrap();
        self.current = &self.current[c.len_utf8()..];
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
        self.generate_token(TokenType::Tstr)
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
        while self.peek().is_ascii_alphabetic()
            || self.peek().is_ascii_digit()
            || self.peek() == '_'
        {
            self.advance();
        }

        let text = &self.start[..self.start.len() - self.current.len()];

        self.generate_token(match text {
            "true" => TokenType::Ttrue,
            "false" => TokenType::Tfalse,
            "print" => TokenType::Tprint,
            "set" => TokenType::Tset,
            "if" => TokenType::Tif,
            "else" => TokenType::Telse,
            "fix" => TokenType::Tfix,
            "and" => TokenType::Tand,
            "or" => TokenType::Tor,
            "while" => TokenType::Twhile,
            "loop" => TokenType::Tloop,
            "stop" => TokenType::Tstop,
            _ => TokenType::Tid,
        })
    }
}
