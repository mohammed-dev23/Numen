use crate::{
    chunk::{Chunk, OpCode, Values},
    scanner::{Scanner, Token, TokenType},
};

#[derive(Debug)]
pub struct Parser<'p> {
    current: Token,
    previous: Token,
    had_err: bool,
    panic_mode: bool,
    chunk: &'p mut Chunk,
    scanner: Scanner<'p>,
}

pub struct ParseRules {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub prec: Precedence,
}

pub type ParseFn = fn(&mut Parser);

const NONE_RULE: ParseRules = ParseRules {
    prefix: None,
    infix: None,
    prec: Precedence::None,
};

// the index here does not start from 0 as the scanner TokenType enum does
// it starts from one so be carful with that
static RULES: [ParseRules; 18] = [
    ParseRules {
        prefix: Some(grouping),
        infix: None,
        prec: Precedence::None,
    },
    NONE_RULE,
    ParseRules {
        prefix: None,
        infix: Some(binary),
        prec: Precedence::Terms,
    },
    ParseRules {
        prefix: Some(unary),
        infix: Some(binary),
        prec: Precedence::Terms,
    },
    NONE_RULE,
    ParseRules {
        prefix: Some(int_num),
        infix: None,
        prec: Precedence::None,
    },
    ParseRules {
        prefix: Some(float_num),
        infix: None,
        prec: Precedence::None,
    },
    NONE_RULE,
    NONE_RULE,
    NONE_RULE,
    NONE_RULE,
    ParseRules {
        prefix: Some(binary),
        infix: Some(binary),
        prec: Precedence::Factors,
    },
    ParseRules {
        prefix: Some(binary),
        infix: Some(binary),
        prec: Precedence::Factors,
    },
    ParseRules {
        prefix: Some(binary),
        infix: Some(binary),
        prec: Precedence::Factors,
    },
    ParseRules {
        prefix: Some(binary),
        infix: Some(binary),
        prec: Precedence::Factors,
    },
    ParseRules {
        prefix: Some(binary),
        infix: Some(binary),
        prec: Precedence::Factors,
    },
    ParseRules {
        prefix: Some(bool_ture),
        infix: None,
        prec: Precedence::None,
    },
    ParseRules {
        prefix: Some(bool_false),
        infix: None,
        prec: Precedence::None,
    },
];
#[derive(Debug, Clone, Copy)]
pub enum Precedence {
    None,
    Assignment, // =
    Eq,         // == !=
    Comps,      // > < >= <=
    Terms,      // - +
    Factors,    // * / % ^ //
    Unary,      // ! -
    Call,       // . ()
    Prime,
}

pub fn new_parser<'p>(chunk: &'p mut Chunk, source: &'p str) -> Parser<'p> {
    let dummy_token = Token {
        token_type: TokenType::TEof,
        start: String::new(),
        len: 0,
        line: 0,
    };

    let scanner = Scanner::new_scanner(source);

    Parser {
        current: dummy_token.clone(),
        previous: dummy_token,
        had_err: false,
        panic_mode: false,
        chunk,
        scanner,
    }
}
pub fn advance(parser: &mut Parser) {
    parser.previous = parser.current.clone();

    loop {
        parser.current = parser.scanner.scan_tokens();

        if parser.current.token_type != TokenType::TErr {
            break;
        }

        error_at_current(parser, &parser.current.start.clone());
    }
}

pub fn error_at_current(parser: &mut Parser, message: &str) {
    let token = parser.current.clone();
    error_at(parser, token, message);
}

pub fn error(parser: &mut Parser, message: &str) {
    let token = parser.previous.clone();
    error_at(parser, token, message);
}

pub fn error_at(parser: &mut Parser, token: Token, message: &str) {
    if parser.panic_mode {
        return;
    };

    parser.panic_mode = true;
    print!("[{:?} , {} Error]", token, token.line);

    if token.token_type == TokenType::TEof {
        print!(" at end");
    } else if token.token_type == TokenType::TErr {
        //idk
    } else {
        print!(" at '{}' , '{}' ", token.len, token.start)
    }

    print!(": {}", message);
    parser.had_err = true;
}

pub fn consume(parser: &mut Parser, message: &str, t_type: TokenType) {
    if parser.current.token_type == t_type {
        advance(parser);
        return;
    }

    error_at_current(parser, message);
}

fn end_compile(parser: &mut Parser) {
    emit_return(parser);
}

fn emit_return(parser: &mut Parser) {
    emit_byte(parser, OpCode::OpR as u8);
}

fn emit_byte(parser: &mut Parser, byte: u8) {
    parser.chunk.write_chunk(byte, parser.previous.line);
}

fn expression(parser: &mut Parser) {
    parse_precedence(parser, Precedence::Assignment);
}

fn emit_bytes(parser: &mut Parser, byte1: u8, byte2: u8) {
    emit_byte(parser, byte1);
    emit_byte(parser, byte2);
}

fn int_num(parser: &mut Parser) {
    let value = parser.previous.start.trim().parse::<i64>().unwrap();
    emit_constant(parser, Values::Int(value));
}

fn float_num(parser: &mut Parser) {
    let value = parser.previous.start.trim().parse::<f64>().unwrap();
    emit_constant(parser, Values::Float(value));
}

fn bool_ture(parser: &mut Parser) {
    emit_constant(parser, Values::Bool(true));
}

fn bool_false(parser: &mut Parser) {
    emit_constant(parser, Values::Bool(false));
}

fn emit_constant(parser: &mut Parser, value: Values) {
    let make = make_constant(parser, value);
    emit_bytes(parser, OpCode::OpC as u8, make);
}

fn make_constant(parser: &mut Parser, value: Values) -> u8 {
    let cons = parser.chunk.add_constant(value);

    if cons as u8 > u8::MAX {
        error(parser, "Too many constants in one chunk.");
        return 0;
    }

    cons as u8
}

fn grouping(parser: &mut Parser) {
    expression(parser);
    consume(parser, "Expect ')' after expression.", TokenType::TRr);
}

fn parse_precedence(parser: &mut Parser, prec: Precedence) {
    advance(parser);

    let prefix_rules = get_rules(parser.previous.token_type).prefix;

    if let Some(p_rules) = prefix_rules {
        p_rules(parser);
    } else {
        error(parser, "Expect expression.");
        return;
    }

    while prec as usize <= get_rules(parser.current.token_type).prec as usize {
        advance(parser);
        let infix_rules = get_rules(parser.previous.token_type).infix;

        if let Some(i_rules) = infix_rules {
            i_rules(parser)
        }
    }
}

fn unary(parser: &mut Parser) {
    let optype = parser.previous.token_type;

    parse_precedence(parser, Precedence::Unary);

    if optype == TokenType::Tminus {
        emit_byte(parser, OpCode::OpNegate as u8)
    }
}

fn binary(parser: &mut Parser) {
    let optype = parser.previous.token_type;
    let rule = get_rules(optype);

    let next_rule = match rule.prec {
        Precedence::None => Precedence::Assignment,
        Precedence::Assignment => Precedence::Eq,
        Precedence::Eq => Precedence::Comps,
        Precedence::Comps => Precedence::Terms,
        Precedence::Terms => Precedence::Factors,
        Precedence::Factors => Precedence::Unary,
        Precedence::Unary => Precedence::Call,
        Precedence::Call => Precedence::Prime,
        Precedence::Prime => Precedence::Prime,
    };

    parse_precedence(parser, next_rule);

    match optype {
        TokenType::TPlus => emit_byte(parser, OpCode::OpAdd as u8),
        TokenType::Tminus => emit_byte(parser, OpCode::OpSubtract as u8),
        TokenType::TdivOp => emit_byte(parser, OpCode::OpDivide as u8),
        TokenType::TmulOp => emit_byte(parser, OpCode::OpMultiply as u8),
        TokenType::TmodOp => emit_byte(parser, OpCode::OpMod as u8),
        TokenType::TpowOp => emit_byte(parser, OpCode::OpPow as u8),
        TokenType::TdivdivOp => emit_byte(parser, OpCode::OpDivideDivide as u8),
        TokenType::Ttrue => emit_byte(parser, OpCode::OpTrue as u8),
        TokenType::Tfalse => emit_byte(parser, OpCode::OpFalse as u8),
        _ => unreachable!(),
    }
}

fn get_rules(t_type: TokenType) -> &'static ParseRules {
    &RULES[t_type as usize]
}

pub fn compile(parser: &mut Parser) -> bool {
    advance(parser);

    loop {
        while parser.current.token_type == TokenType::TSemicolon {
            advance(parser);
        }

        if parser.current.token_type == TokenType::TEof {
            break;
        }

        expression(parser);

        consume(
            parser,
            "Expect ';' after expression.",
            TokenType::TSemicolon,
        );

        end_compile(parser);
    }

    #[cfg(feature = "dbte")]
    {
        if parser.had_err {
            parser.chunk.disassembler("code");
        }
    }

    consume(parser, "Expect end of expression.", TokenType::TEof);
    !parser.had_err
}
