use crate::chunk::{Chunk, OpCode, Values};
use std::f64;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Values>,
}

pub enum InterpretResult {
    InterpretOK,
    InterpretCompileErr,
    _InterpretRunTimeErr,
}

pub enum BinaryOp {
    Add,
    Subtract,
    Divide,
    Multiply,
    Mod,
    Pow,
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
}

pub enum ComparisonOp {
    Eq,
    NotEq,
    Gt,
    Lt,
    Gte,
    Lte,
}

impl VM {
    pub fn new_vm(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, _source: String) -> InterpretResult {
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

            let instruction = self.read_bytes();

            match instruction {
                i if i == OpCode::OpR as u8 => {
                    let v = self.stack.pop().unwrap();
                    println!(" {:?} ", v);
                    return InterpretResult::InterpretOK;
                }

                i if i == OpCode::OpC as u8 => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                    continue;
                }

                i if i == OpCode::OpAdd as u8 => {
                    self.binary_op(BinaryOp::Add);
                    continue;
                }

                i if i == OpCode::OpSubtract as u8 => {
                    self.binary_op(BinaryOp::Subtract);
                    continue;
                }

                i if i == OpCode::OpDivide as u8 => {
                    self.binary_op(BinaryOp::Divide);
                    continue;
                }

                i if i == OpCode::OpMultiply as u8 => {
                    self.binary_op(BinaryOp::Multiply);
                    continue;
                }

                i if i == OpCode::OpMod as u8 => {
                    self.binary_op(BinaryOp::Mod);
                    continue;
                }

                i if i == OpCode::OpPow as u8 => {
                    self.binary_op(BinaryOp::Pow);
                    continue;
                }

                i if i == OpCode::OpNegate as u8 => {
                    self.unary_op(UnaryOp::Negate);
                    continue;
                }

                i if i == OpCode::OpSqrt as u8 => {
                    self.unary_op(UnaryOp::Sqrt);
                    continue;
                }

                i if i == OpCode::OpAbs as u8 => {
                    self.unary_op(UnaryOp::Abs);
                    continue;
                }

                i if i == OpCode::OpFloor as u8 => {
                    self.unary_op(UnaryOp::Floor);
                    continue;
                }

                i if i == OpCode::OpCeil as u8 => {
                    self.unary_op(UnaryOp::Ceil);
                    continue;
                }

                i if i == OpCode::OpSin as u8 => {
                    self.unary_op(UnaryOp::Sin);
                    continue;
                }

                i if i == OpCode::OpCos as u8 => {
                    self.unary_op(UnaryOp::Cos);
                    continue;
                }

                i if i == OpCode::OpTan as u8 => {
                    self.unary_op(UnaryOp::Tan);
                    continue;
                }

                i if i == OpCode::OpEq as u8 => {
                    self.comparison_op(ComparisonOp::Eq);
                    continue;
                }

                i if i == OpCode::OpNotEq as u8 => {
                    self.comparison_op(ComparisonOp::NotEq);
                    continue;
                }

                i if i == OpCode::OpGt as u8 => {
                    self.comparison_op(ComparisonOp::Gt);
                    continue;
                }

                i if i == OpCode::OpLt as u8 => {
                    self.comparison_op(ComparisonOp::Lt);
                    continue;
                }

                i if i == OpCode::OpGte as u8 => {
                    self.comparison_op(ComparisonOp::Gte);
                    continue;
                }

                i if i == OpCode::OpLte as u8 => {
                    self.comparison_op(ComparisonOp::Lte);
                    continue;
                }

                _ => {
                    println!("! InterpretCompileErr !");
                    return InterpretResult::InterpretCompileErr;
                }
            }
        }
    }

    fn read_bytes(&mut self) -> u8 {
        let bytes = self.chunk.code[self.ip];
        self.ip += 1;
        bytes
    }

    fn read_constant(&mut self) -> Values {
        let index = self.read_bytes() as usize;
        self.chunk.constant.values[index]
    }

    fn binary_op(&mut self, op: BinaryOp) {
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
            },
            (Values::Float(x), Values::Float(y)) => match op {
                BinaryOp::Add => self.stack.push(Values::Float(x + y)),
                BinaryOp::Subtract => self.stack.push(Values::Float(x - y)),
                BinaryOp::Divide => self.stack.push(Values::Float(x / y)),
                BinaryOp::Multiply => self.stack.push(Values::Float(x * y)),
                BinaryOp::Mod => self.stack.push(Values::Float(x % y)),
                BinaryOp::Pow => self.stack.push(Values::Float(x.powf(y))),
            },
            (Values::Float(x), Values::Int(y)) => match op {
                BinaryOp::Add => self.stack.push(Values::Float(x + y as f64)),
                BinaryOp::Subtract => self.stack.push(Values::Float(x - y as f64)),
                BinaryOp::Divide => self.stack.push(Values::Float(x / y as f64)),
                BinaryOp::Multiply => self.stack.push(Values::Float(x * y as f64)),
                BinaryOp::Mod => self.stack.push(Values::Float(x % y as f64)),
                BinaryOp::Pow => self.stack.push(Values::Float(x.powf(y as f64))),
            },
            (Values::Int(c), Values::Float(b)) => match op {
                BinaryOp::Add => self.stack.push(Values::Float(c as f64 + b)),
                BinaryOp::Subtract => self.stack.push(Values::Float(c as f64 - b)),
                BinaryOp::Divide => self.stack.push(Values::Float(c as f64 / b)),
                BinaryOp::Multiply => self.stack.push(Values::Float(c as f64 * b)),
                BinaryOp::Mod => self.stack.push(Values::Float(c as f64 % b)),
                BinaryOp::Pow => self.stack.push(Values::Float((c as f64).powf(b))),
            },

            _ => panic!("unsupported operation"),
        }
    }

    fn unary_op(&mut self, unary_op: UnaryOp) {
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
            },

            _ => panic!("unsupported operation"),
        }
    }

    fn comparison_op(&mut self, op: ComparisonOp) {
        let y = self.stack.pop().unwrap();
        let x = self.stack.pop().unwrap();

        match (x, y) {
            (Values::Int(x), Values::Int(y)) => match op {
                ComparisonOp::Eq => self.stack.push(Values::Bool(x == y)),
                ComparisonOp::NotEq => self.stack.push(Values::Bool(x != y)),
                ComparisonOp::Gt => self.stack.push(Values::Bool(x > y)),
                ComparisonOp::Lt => self.stack.push(Values::Bool(x < y)),
                ComparisonOp::Gte => self.stack.push(Values::Bool(x >= y)),
                ComparisonOp::Lte => self.stack.push(Values::Bool(x <= y)),
            },
            (Values::Float(x), Values::Float(y)) => match op {
                ComparisonOp::Eq => self.stack.push(Values::Bool(x == y)),
                ComparisonOp::NotEq => self.stack.push(Values::Bool(x != y)),
                ComparisonOp::Gt => self.stack.push(Values::Bool(x > y)),
                ComparisonOp::Lt => self.stack.push(Values::Bool(x < y)),
                ComparisonOp::Gte => self.stack.push(Values::Bool(x >= y)),
                ComparisonOp::Lte => self.stack.push(Values::Bool(x <= y)),
            },
            (Values::Int(x), Values::Float(y)) => match op {
                ComparisonOp::Eq => self.stack.push(Values::Bool(x as f64 == y)),
                ComparisonOp::NotEq => self.stack.push(Values::Bool(x as f64 != y)),
                ComparisonOp::Gt => self.stack.push(Values::Bool(x as f64 > y)),
                ComparisonOp::Lt => self.stack.push(Values::Bool((x as f64) < y)),
                ComparisonOp::Gte => self.stack.push(Values::Bool(x as f64 >= y)),
                ComparisonOp::Lte => self.stack.push(Values::Bool(x as f64 <= y)),
            },
            (Values::Float(x), Values::Int(y)) => match op {
                ComparisonOp::Eq => self.stack.push(Values::Bool(x == y as f64)),
                ComparisonOp::NotEq => self.stack.push(Values::Bool(x != y as f64)),
                ComparisonOp::Gt => self.stack.push(Values::Bool(x > y as f64)),
                ComparisonOp::Lt => self.stack.push(Values::Bool(x < y as f64)),
                ComparisonOp::Gte => self.stack.push(Values::Bool(x >= y as f64)),
                ComparisonOp::Lte => self.stack.push(Values::Bool(x <= y as f64)),
            },

            _ => panic!("unsupported operation"),
        }
    }
}
