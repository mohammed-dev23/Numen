use crate::{
    chunk_values::{Chunk, OpCode, Values},
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
    compiler: Option<Box<Compiler>>,
}

pub struct ParseRules {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub prec: Precedence,
}

#[derive(Debug)]
pub struct Compiler {
    locals: Vec<Local>,
    local_count: i64,
    scope_depth: i64,
}

#[derive(Debug)]
pub struct Local {
    name: Token,
    depth: i64,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            locals: Vec::new(),
            local_count: 0,
            scope_depth: 0,
        }
    }

    fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn end_scope(&mut self) -> u8 {
        self.scope_depth -= 1;
        let mut pops: u8 = 0;

        while self.local_count > 0
            && self.locals[(self.local_count - 1) as usize].depth > self.scope_depth
        {
            pops += 1;
            self.local_count -= 1;
        }

        pops
    }

    fn add_local(&mut self, name: Token) {
        let local = Local {
            name: name,
            depth: -1,
        };
        self.locals.push(local);
        self.local_count += 1;
    }

    fn mark_initialized(&mut self) {
        let last = self.local_count as usize - 1;
        self.locals[last].depth = self.scope_depth;
    }
}

pub type ParseFn = fn(&mut Parser, can_assign: bool);

const NONE_RULE: ParseRules = ParseRules {
    prefix: None,
    infix: None,
    prec: Precedence::None,
};

// the index here does not start from 0 as the scanner TokenType enum does
// it starts from one so be carful with that
static RULES: [ParseRules; 32] = [
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
    ParseRules {
        prefix: Some(strings),
        infix: None,
        prec: Precedence::None,
    },
    ParseRules {
        prefix: Some(variable),
        infix: None,
        prec: Precedence::None,
    },
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
    ParseRules {
        prefix: Some(unary),
        infix: None,
        prec: Precedence::Unary,
    },
    ParseRules {
        prefix: None,
        infix: Some(binary),
        prec: Precedence::Eq,
    },
    NONE_RULE,
    ParseRules {
        prefix: None,
        infix: Some(binary),
        prec: Precedence::Eq,
    },
    ParseRules {
        prefix: None,
        infix: Some(binary),
        prec: Precedence::Comps,
    },
    ParseRules {
        prefix: None,
        infix: Some(binary),
        prec: Precedence::Comps,
    },
    ParseRules {
        prefix: None,
        infix: Some(binary),
        prec: Precedence::Comps,
    },
    ParseRules {
        prefix: None,
        infix: Some(binary),
        prec: Precedence::Comps,
    },
    NONE_RULE,
    NONE_RULE,
    NONE_RULE,
    NONE_RULE,
    NONE_RULE,
    NONE_RULE,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
        compiler: Some(Box::new(Compiler::new())),
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

fn int_num(parser: &mut Parser, _can_assign: bool) {
    let value = parser.previous.start.trim().parse::<i64>().unwrap();
    emit_constant(parser, Values::Int(value));
}

fn float_num(parser: &mut Parser, _can_assign: bool) {
    let value = parser.previous.start.trim().parse::<f64>().unwrap();
    emit_constant(parser, Values::Float(value));
}

fn strings<'p>(parser: &mut Parser<'p>, _can_assign: bool) {
    let raw_value = &parser.previous.start;
    let value = Values::Str(raw_value[1..raw_value.len() - 1].into()); // strips the \str\
    emit_constant(parser, value);
}

fn bool_ture(parser: &mut Parser, _can_assign: bool) {
    emit_constant(parser, Values::Bool(true));
}

fn bool_false(parser: &mut Parser, _can_assign: bool) {
    emit_constant(parser, Values::Bool(false));
}

fn variable(parser: &mut Parser, can_assign: bool) {
    named_variable(parser, parser.previous.clone(), can_assign);
}

fn named_variable(parser: &mut Parser, name: Token, can_assign: bool) {
    let arg = resolve_locals(parser, &name);

    let get_op: u8;
    let set_op: u8;

    let arg = if arg != -1 {
        get_op = OpCode::OpGetLocal as u8;
        set_op = OpCode::OpSetLocal as u8;

        arg as u8
    } else {
        let arg = ider_constant(parser, name);
        get_op = OpCode::OpGetGlobal as u8;
        set_op = OpCode::OpSetGlobal as u8;

        arg
    };

    if can_assign && match_tokens(parser, TokenType::Teq) {
        expression(parser);
        emit_bytes(parser, set_op, arg);
    } else {
        emit_bytes(parser, get_op, arg);
    }
}

fn resolve_locals(parser: &mut Parser, name: &Token) -> i64 {
    for i in (0..parser.compiler.as_ref().unwrap().local_count).rev() {
        let local = &parser.compiler.as_ref().unwrap().locals[i as usize].name;
        let depth = &parser.compiler.as_ref().unwrap().locals[i as usize].depth;

        if ider_eq(&name, local) {
            if *depth == -1 {
                error(parser, "Can't read local variable in its own initializer.");
            }
            return i;
        }
    }
    return -1;
}

fn emit_constant<'p>(parser: &mut Parser<'p>, value: Values) {
    let make = make_constant(parser, value);
    emit_bytes(parser, OpCode::OpC as u8, make);
}

fn make_constant<'p>(parser: &mut Parser<'p>, value: Values) -> u8 {
    let cons = parser.chunk.add_constant(value);

    if cons > u8::MAX as usize {
        error(parser, "Too many constants in one chunk.");
        return 0;
    }

    cons as u8
}

fn grouping(parser: &mut Parser, _can_assign: bool) {
    expression(parser);
    consume(parser, "Expect ')' after expression.", TokenType::TRp);
}

fn parse_precedence(parser: &mut Parser, prec: Precedence) {
    advance(parser);

    let prefix_rules = get_rules(parser.previous.token_type).prefix;

    if let Some(p_rules) = prefix_rules {
        let can_assign = prec <= Precedence::Assignment;
        p_rules(parser, can_assign);
    } else {
        error(parser, "Expect expression.");
        return;
    }

    while prec as usize <= get_rules(parser.current.token_type).prec as usize {
        advance(parser);
        let infix_rules = get_rules(parser.previous.token_type).infix;

        if let Some(i_rules) = infix_rules {
            let can_assign = prec <= Precedence::Assignment;
            i_rules(parser, can_assign)
        }
    }
    let can_assign = prec <= Precedence::Assignment;
    if can_assign && match_tokens(parser, TokenType::Teq) {
        error(parser, "Invalid assignment target.");
    }
}

fn unary(parser: &mut Parser, _can_assign: bool) {
    let optype = parser.previous.token_type;

    parse_precedence(parser, Precedence::Unary);

    match optype {
        TokenType::Tminus => emit_byte(parser, OpCode::OpNegate as u8),
        TokenType::Tnot => emit_byte(parser, OpCode::OpNot as u8),
        _ => error(parser, "unsupported operand."),
    }
}

fn binary(parser: &mut Parser, _can_assign: bool) {
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
        TokenType::Teqeq => emit_byte(parser, OpCode::OpEqEq as u8),
        TokenType::TnotEq => emit_byte(parser, OpCode::OpNotEq as u8),
        TokenType::Tgt => emit_byte(parser, OpCode::OpGt as u8),
        TokenType::Tlt => emit_byte(parser, OpCode::OpLt as u8),
        TokenType::Tgte => emit_byte(parser, OpCode::OpGte as u8),
        TokenType::Tlte => emit_byte(parser, OpCode::OpLte as u8),
        _ => unreachable!(),
    }
}

fn get_rules(t_type: TokenType) -> &'static ParseRules {
    &RULES[t_type as usize]
}

fn declaration(parser: &mut Parser) {
    if match_tokens(parser, TokenType::Tmake) {
        var_declaration(parser);
    } else {
        statement(parser);
    }

    if parser.panic_mode {
        sync(parser);
    }
}

fn statement(parser: &mut Parser) {
    if match_tokens(parser, TokenType::Tprint) {
        print_statement(parser);
    } else if match_tokens(parser, TokenType::Tif) {
        if_statement(parser);
    } else if match_tokens(parser, TokenType::Tlb) {
        parser.compiler.as_mut().unwrap().begin_scope();
        block(parser);
        let pops = parser.compiler.as_mut().unwrap().end_scope();

        for _ in 0..pops {
            emit_byte(parser, OpCode::OpPop as u8);
        }
    } else {
        expression_statement(parser);
    }
}

fn block(parser: &mut Parser) {
    while !check(parser, TokenType::Trb) && !check(parser, TokenType::TEof) {
        declaration(parser);
    }

    consume(parser, "Expect '}' after block.", TokenType::Trb);
}

fn print_statement(parser: &mut Parser) {
    expression(parser);
    consume(parser, "Expect ';' after value.", TokenType::TSemicolon);
    emit_byte(parser, OpCode::OpPrint as u8);
}

fn if_statement(parser: &mut Parser) {
    expression(parser);

    let then_jump = emit_jump(parser, OpCode::OpJumpIfFalse as u8);
    statement(parser);

    patch_jump(parser, then_jump);
}

fn emit_jump(parser: &mut Parser, instruction: u8) -> usize {
    emit_byte(parser, instruction);
    emit_byte(parser, 0xff);
    emit_byte(parser, 0xff);
    parser.chunk.code.len() - 2
}

fn patch_jump(parser: &mut Parser, offset: usize) {
    let jump = parser.chunk.code.len() - offset - 2;
    parser.chunk.code[offset] = ((jump >> 8) & 0xff) as u8;
    parser.chunk.code[offset + 1] = (jump & 0xff) as u8;
}

fn var_declaration(parser: &mut Parser) {
    let global = parse_var(parser, "Expect variable name.");

    if match_tokens(parser, TokenType::Teq) {
        expression(parser);
    } else {
        error(parser, "Expect '=' after variable name.");
    }
    consume(
        parser,
        "Expect ';' after variable declaration.",
        TokenType::TSemicolon,
    );
    define_var(parser, global);
}

fn define_var(parser: &mut Parser, global: u8) {
    if parser.compiler.as_ref().unwrap().scope_depth > 0 {
        parser.compiler.as_mut().unwrap().mark_initialized();
        return;
    }
    emit_bytes(parser, OpCode::OpDefGlobal as u8, global);
}

fn declare_var(parser: &mut Parser) {
    if parser.compiler.as_ref().unwrap().scope_depth == 0 {
        return;
    }

    let name = parser.previous.clone();

    for i in (0..parser.compiler.as_ref().unwrap().local_count).rev() {
        let local = &parser.compiler.as_mut().unwrap().locals[i as usize]
            .depth
            .clone();

        let local_name = &parser.compiler.as_ref().unwrap().locals[i as usize].name;

        if *local != -1 && *local < parser.compiler.as_deref().unwrap().scope_depth as i64 {
            break;
        }

        if ider_eq(&name, &local_name) {
            error(parser, "Already a variable with this name in this scope.");
        }
    }

    parser.compiler.as_mut().unwrap().add_local(name.clone());
}

fn ider_eq(v1: &Token, v2: &Token) -> bool {
    v1.start == v2.start
}

fn parse_var(parser: &mut Parser, message: &str) -> u8 {
    consume(parser, message, TokenType::TId);

    declare_var(parser);
    if parser.compiler.as_ref().unwrap().scope_depth > 0 {
        return 0;
    }

    ider_constant(parser, parser.previous.clone())
}

fn ider_constant(parser: &mut Parser, name: Token) -> u8 {
    let value = Values::Str(name.start.into());
    make_constant(parser, value)
}

fn expression_statement(parser: &mut Parser) {
    expression(parser);
    consume(parser, "Expect ';' after value.", TokenType::TSemicolon);
    emit_byte(parser, OpCode::OpPop as u8);
}

fn match_tokens(parser: &mut Parser, t_type: TokenType) -> bool {
    if !check(parser, t_type) {
        return false;
    }
    advance(parser);
    true
}

fn check(parser: &mut Parser, t_type: TokenType) -> bool {
    parser.current.token_type == t_type
}

fn sync(parser: &mut Parser) {
    parser.panic_mode = false;

    while parser.current.token_type != TokenType::TEof {
        if parser.previous.token_type == TokenType::TSemicolon {
            return;
        }
        match parser.current.token_type {
            TokenType::Tprint | TokenType::Tmake => return,
            _ => return,
        }
    }
}

pub fn compile(parser: &mut Parser) -> bool {
    advance(parser);

    while !match_tokens(parser, TokenType::TEof) {
        declaration(parser);
    }

    end_compile(parser);

    #[cfg(feature = "dbte")]
    {
        if !parser.had_err {
            parser.chunk.disassembler("code");
        }
    }

    !parser.had_err
}
