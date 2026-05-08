use crate::{
    chunk_values::{Chunk, OpCode, Values},
    compiler::{compile, new_parser},
    table::Table,
};

use std::rc::Rc;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Values>,
    globals: Table,
}

pub enum InterpretResult {
    InterpretOK,
    InterpretCompileErr,
    InterpretRunTimeErr,
}

pub enum BinaryOp {
    Add,
    Subtract,
    Divide,
    Multiply,
    Mod,
    Pow,
    DivideDivide,
}

pub enum UnaryOp {
    Negate,
    Sqrt,
    Abs,
    Floor,
    Ceil,
    Sin,
    Cos,
    Tan,
    Not,
}

pub enum ComparisonOp {
    EqEq,
    NotEq,
    Gt,
    Lt,
    Gte,
    Lte,
}

pub enum LogicalOp {
    And,
    Or,
}

impl VM {
    pub fn new_vm(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
            globals: Table::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk = self.chunk.clone();
        let mut parser = new_parser(&mut chunk, source);

        if !compile(&mut parser) {
            return InterpretResult::InterpretCompileErr;
        };

        self.chunk = chunk;
        self.ip = 0;
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature = "dbte")]
            {
                let offset_for_debug = self.ip;
                print!("      ");

                for i in &self.stack {
                    print!("[ {:?} ]", i)
                }

                println!();
                self.chunk.disassembler_instruction(offset_for_debug);
            }

            if self.ip >= self.chunk.code.len() {
                return InterpretResult::InterpretOK;
            }

            let instruction = self.read_bytes();

            match instruction {
                i if i == OpCode::OpR as u16 => {
                    return InterpretResult::InterpretOK;
                }

                i if i == OpCode::OpC as u16 => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                    continue;
                }

                i if i == OpCode::OpAdd as u16 => {
                    if !self.binary_op(BinaryOp::Add) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpSubtract as u16 => {
                    if !self.binary_op(BinaryOp::Subtract) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpDivide as u16 => {
                    if !self.binary_op(BinaryOp::Divide) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpMultiply as u16 => {
                    if !self.binary_op(BinaryOp::Multiply) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpMod as u16 => {
                    if !self.binary_op(BinaryOp::Mod) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpPow as u16 => {
                    if !self.binary_op(BinaryOp::Pow) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpNegate as u16 => {
                    if !self.unary_op(UnaryOp::Negate) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpSqrt as u16 => {
                    if !self.unary_op(UnaryOp::Sqrt) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpAbs as u16 => {
                    if !self.unary_op(UnaryOp::Abs) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpFloor as u16 => {
                    if !self.unary_op(UnaryOp::Floor) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpCeil as u16 => {
                    if !self.unary_op(UnaryOp::Ceil) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpSin as u16 => {
                    if !self.unary_op(UnaryOp::Sin) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpCos as u16 => {
                    if !self.unary_op(UnaryOp::Cos) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpTan as u16 => {
                    if !self.unary_op(UnaryOp::Tan) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpEqEq as u16 => {
                    if !self.comparison_op(ComparisonOp::EqEq) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpNotEq as u16 => {
                    if !self.comparison_op(ComparisonOp::NotEq) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpGt as u16 => {
                    if !self.comparison_op(ComparisonOp::Gt) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpLt as u16 => {
                    if !self.comparison_op(ComparisonOp::Lt) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpGte as u16 => {
                    if !self.comparison_op(ComparisonOp::Gte) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpLte as u16 => {
                    if !self.comparison_op(ComparisonOp::Lte) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpDivideDivide as u16 => {
                    if !self.binary_op(BinaryOp::DivideDivide) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpNot as u16 => {
                    if !self.unary_op(UnaryOp::Not) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpPrint as u16 => {
                    let value = self.stack.pop().unwrap();
                    println!("{}", value);
                    continue;
                }

                i if i == OpCode::OpPop as u16 => {
                    self.stack.pop().unwrap();
                    continue;
                }

                i if i == OpCode::OpDefGlobal as u16 => {
                    let name = self.read_constant().to_string();
                    self.globals
                        .set_table(&name, self.stack.pop().unwrap(), true);
                    continue;
                }

                i if i == OpCode::OpDefFixed as u16 => {
                    let name = self.read_constant().to_string();
                    self.globals
                        .set_table(&name, self.stack.pop().unwrap(), false);
                    continue;
                }

                i if i == OpCode::OpGetGlobal as u16 => {
                    let name = self.read_constant().to_string();

                    if let Some((value, _)) = self.globals.get_value(&name) {
                        let value = value;
                        self.stack.push(value.clone());
                        continue;
                    } else {
                        self.runtime_errors("Undefined variable");
                        print!("{}", name);
                        return InterpretResult::InterpretRunTimeErr;
                    }
                }

                i if i == OpCode::OpSetGlobal as u16 => {
                    let name = self.read_constant().to_string();

                    match self.globals.get_value(&name) {
                        None => {
                            self.globals.delete(&name);
                            self.runtime_errors("Undefined variable");
                            print!("{}", name);
                            return InterpretResult::InterpretRunTimeErr;
                        }
                        Some((_, false)) => {
                            self.runtime_errors("Cannot reassign immutable variable.");
                            return InterpretResult::InterpretRunTimeErr;
                        }
                        Some((_, true)) => {
                            let value = self.stack.last().unwrap().clone();
                            self.globals.set_table(&name, value, true);
                        }
                    }
                    continue;
                }

                i if i == OpCode::OpGetLocal as u16 => {
                    let slot = self.read_bytes();
                    let value = self.stack[slot as usize].clone();
                    self.stack.push(value);
                    continue;
                }

                i if i == OpCode::OpSetLocalFixed as u16 => {
                    let _slot = self.read_bytes();
                    self.runtime_errors("Cannot reassign immutable variable.");
                    return InterpretResult::InterpretRunTimeErr;
                }

                i if i == OpCode::OpSetLocal as u16 => {
                    let slot = self.read_bytes();
                    self.stack[slot as usize] = self.stack.last().unwrap().clone();

                    continue;
                }

                i if i == OpCode::OpJumpIfFalse as u16 => {
                    let offset = self.read_short() as usize;
                    let value = self.stack.last().unwrap();

                    if value.is_false() {
                        self.ip += offset;
                        continue;
                    }
                }

                i if i == OpCode::OpAnd as u16 => {
                    if !self.logical_op(LogicalOp::And) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpOr as u16 => {
                    if !self.logical_op(LogicalOp::Or) {
                        return InterpretResult::InterpretRunTimeErr;
                    }
                    continue;
                }

                i if i == OpCode::OpJump as u16 => {
                    let offset = self.read_short();
                    self.ip += offset as usize;
                    continue;
                }

                i if i == OpCode::OpLoop as u16 => {
                    let offset = self.read_short();
                    self.ip -= offset as usize;
                    continue;
                }

                _ => {
                    println!("! InterpretCompileErr !");
                    return InterpretResult::InterpretCompileErr;
                }
            }
        }
    }

    fn read_bytes(&mut self) -> u16 {
        let bytes = self.chunk.code[self.ip];
        self.ip += 1;
        bytes
    }

    fn read_constant(&mut self) -> Values {
        let index = self.read_bytes() as usize;
        self.chunk.constant.values[index].clone()
    }

    fn read_short(&mut self) -> u16 {
        let high = self.chunk.code[self.ip] as u16;
        let low = self.chunk.code[self.ip + 1] as u16;

        self.ip += 2;
        (high << 8) | low
    }

    fn runtime_errors(&mut self, msg: &str) {
        eprintln!("{}", msg);

        let instruction = self.ip - 1;
        let line = self.chunk.line[instruction];
        eprintln!("[line {}] in script", line);

        self.stack.clear();
    }

    fn binary_op(&mut self, op: BinaryOp) -> bool {
        let y = self.stack.pop().unwrap();
        let x = self.stack.pop().unwrap();

        match (x, y) {
            (Values::Int(x), Values::Int(y)) => match op {
                BinaryOp::Add => self.stack.push(Values::Int(x + y)),
                BinaryOp::Subtract => self.stack.push(Values::Int(x - y)),
                BinaryOp::Divide => self.stack.push(Values::Int(x / y)),
                BinaryOp::Multiply => self.stack.push(Values::Int(x * y)),
                BinaryOp::Mod => self.stack.push(Values::Int(x % y)),
                BinaryOp::Pow => self.stack.push(Values::Int(x.pow(y as u32))),
                BinaryOp::DivideDivide => self
                    .stack
                    .push(Values::Int(((x / y) as f64).floor() as i64)),
            },
            (Values::Float(x), Values::Float(y)) => match op {
                BinaryOp::Add => self.stack.push(Values::Float(x + y)),
                BinaryOp::Subtract => self.stack.push(Values::Float(x - y)),
                BinaryOp::Divide => self.stack.push(Values::Float(x / y)),
                BinaryOp::Multiply => self.stack.push(Values::Float(x * y)),
                BinaryOp::Mod => self.stack.push(Values::Float(x % y)),
                BinaryOp::Pow => self.stack.push(Values::Float(x.powf(y))),
                BinaryOp::DivideDivide => self.stack.push(Values::Float((x / y).floor())),
            },
            (Values::Float(x), Values::Int(y)) => match op {
                BinaryOp::Add => self.stack.push(Values::Float(x + y as f64)),
                BinaryOp::Subtract => self.stack.push(Values::Float(x - y as f64)),
                BinaryOp::Divide => self.stack.push(Values::Float(x / y as f64)),
                BinaryOp::Multiply => self.stack.push(Values::Float(x * y as f64)),
                BinaryOp::Mod => self.stack.push(Values::Float(x % y as f64)),
                BinaryOp::Pow => self.stack.push(Values::Float(x.powf(y as f64))),
                BinaryOp::DivideDivide => self.stack.push(Values::Float((x / y as f64).floor())),
            },
            (Values::Int(x), Values::Float(y)) => match op {
                BinaryOp::Add => self.stack.push(Values::Float(x as f64 + y)),
                BinaryOp::Subtract => self.stack.push(Values::Float(x as f64 - y)),
                BinaryOp::Divide => self.stack.push(Values::Float(x as f64 / y)),
                BinaryOp::Multiply => self.stack.push(Values::Float(x as f64 * y)),
                BinaryOp::Mod => self.stack.push(Values::Float(x as f64 % y)),
                BinaryOp::Pow => self.stack.push(Values::Float((x as f64).powf(y))),
                BinaryOp::DivideDivide => self.stack.push(Values::Float((x as f64 / y).floor())),
            },
            (Values::Str(x), Values::Str(y)) => match op {
                BinaryOp::Add => {
                    let mut x_v = String::new();
                    x_v.push_str(&x);

                    self.stack.push(Values::Str(Rc::from(x_v + &y)));
                }
                _ => {
                    self.runtime_errors("Unsupported operation for strings.");
                    return false;
                }
            },
            _ => {
                self.runtime_errors("Operands must be numbers.");
                return false;
            }
        }
        true
    }

    fn unary_op(&mut self, unary_op: UnaryOp) -> bool {
        let x = self.stack.pop().unwrap();

        match x {
            Values::Int(x) => match unary_op {
                UnaryOp::Negate => self.stack.push(Values::Int(-x)),
                UnaryOp::Sqrt => self.stack.push(Values::Float(f64::sqrt(x as f64))),
                UnaryOp::Abs => self.stack.push(Values::Int(x.abs())),
                UnaryOp::Floor => self.stack.push(Values::Float(f64::floor(x as f64))),
                UnaryOp::Ceil => self.stack.push(Values::Float(f64::ceil(x as f64))),
                UnaryOp::Sin => self.stack.push(Values::Float(f64::sin(x as f64))),
                UnaryOp::Cos => self.stack.push(Values::Float(f64::cos(x as f64))),
                UnaryOp::Tan => self.stack.push(Values::Float(f64::tan(x as f64))),
                UnaryOp::Not => {
                    self.runtime_errors("Operand must be a bool.");
                    return false;
                }
            },
            Values::Float(x) => match unary_op {
                UnaryOp::Negate => self.stack.push(Values::Float(-x)),
                UnaryOp::Sqrt => self.stack.push(Values::Float(x.sqrt())),
                UnaryOp::Abs => self.stack.push(Values::Float(x.abs())),
                UnaryOp::Floor => self.stack.push(Values::Float(x.floor())),
                UnaryOp::Ceil => self.stack.push(Values::Float(x.ceil())),
                UnaryOp::Sin => self.stack.push(Values::Float(x.sin())),
                UnaryOp::Cos => self.stack.push(Values::Float(x.cos())),
                UnaryOp::Tan => self.stack.push(Values::Float(x.tan())),
                UnaryOp::Not => {
                    self.runtime_errors("Operand must be a bool.");
                    return false;
                }
            },
            Values::Bool(x) => match unary_op {
                UnaryOp::Not => {
                    if x == true {
                        self.stack.push(Values::Bool(false));
                    } else {
                        self.stack.push(Values::Bool(true));
                    }
                }
                _ => {
                    self.runtime_errors("Operand must be a bool.");
                    return false;
                }
            },
            _ => {
                self.runtime_errors("Operand must be a number.");
                return false;
            }
        }
        true
    }

    fn comparison_op(&mut self, op: ComparisonOp) -> bool {
        let y = self.stack.pop().unwrap();
        let x = self.stack.pop().unwrap();

        match (x, y) {
            (Values::Int(x), Values::Int(y)) => match op {
                ComparisonOp::EqEq => self.stack.push(Values::Bool(x == y)),
                ComparisonOp::NotEq => self.stack.push(Values::Bool(x != y)),
                ComparisonOp::Gt => self.stack.push(Values::Bool(x > y)),
                ComparisonOp::Lt => self.stack.push(Values::Bool(x < y)),
                ComparisonOp::Gte => self.stack.push(Values::Bool(x >= y)),
                ComparisonOp::Lte => self.stack.push(Values::Bool(x <= y)),
            },
            (Values::Float(x), Values::Float(y)) => match op {
                ComparisonOp::EqEq => self.stack.push(Values::Bool(x == y)),
                ComparisonOp::NotEq => self.stack.push(Values::Bool(x != y)),
                ComparisonOp::Gt => self.stack.push(Values::Bool(x > y)),
                ComparisonOp::Lt => self.stack.push(Values::Bool(x < y)),
                ComparisonOp::Gte => self.stack.push(Values::Bool(x >= y)),
                ComparisonOp::Lte => self.stack.push(Values::Bool(x <= y)),
            },
            (Values::Int(x), Values::Float(y)) => match op {
                ComparisonOp::EqEq => self.stack.push(Values::Bool(x as f64 == y)),
                ComparisonOp::NotEq => self.stack.push(Values::Bool(x as f64 != y)),
                ComparisonOp::Gt => self.stack.push(Values::Bool(x as f64 > y)),
                ComparisonOp::Lt => self.stack.push(Values::Bool((x as f64) < y)),
                ComparisonOp::Gte => self.stack.push(Values::Bool(x as f64 >= y)),
                ComparisonOp::Lte => self.stack.push(Values::Bool(x as f64 <= y)),
            },
            (Values::Float(x), Values::Int(y)) => match op {
                ComparisonOp::EqEq => self.stack.push(Values::Bool(x == y as f64)),
                ComparisonOp::NotEq => self.stack.push(Values::Bool(x != y as f64)),
                ComparisonOp::Gt => self.stack.push(Values::Bool(x > y as f64)),
                ComparisonOp::Lt => self.stack.push(Values::Bool(x < y as f64)),
                ComparisonOp::Gte => self.stack.push(Values::Bool(x >= y as f64)),
                ComparisonOp::Lte => self.stack.push(Values::Bool(x <= y as f64)),
            },
            (Values::Bool(x), Values::Bool(y)) => match op {
                ComparisonOp::EqEq => self.stack.push(Values::Bool(x == y)),
                ComparisonOp::NotEq => self.stack.push(Values::Bool(x != y)),
                _ => {
                    self.runtime_errors("Unsupported compression operator");
                    return false;
                }
            },
            (Values::Str(x), Values::Str(y)) => match op {
                ComparisonOp::EqEq => self.stack.push(Values::Bool(x == y)),
                ComparisonOp::NotEq => self.stack.push(Values::Bool(x != y)),
                _ => {
                    self.runtime_errors("Unsupported compression operation for strings.");
                    return false;
                }
            },
            _ => {
                self.runtime_errors("Unsupported compression operation.");
                return false;
            }
        }
        true
    }
    fn logical_op(&mut self, op: LogicalOp) -> bool {
        let y = self.stack.pop().unwrap();
        let x = self.stack.pop().unwrap();

        match (x, y) {
            (Values::Bool(x), Values::Bool(y)) => match op {
                LogicalOp::And => self.stack.push(Values::Bool(x && y)),
                LogicalOp::Or => self.stack.push(Values::Bool(x || y)),
            },
            _ => {
                self.runtime_errors("Operands must be booleans.");
                return false;
            }
        }
        true
    }
}
