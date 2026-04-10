use crate::chunk::{Chunk, OpCode, Values};
use std::{f64, ops::Fn};

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Values>,
}

pub enum InterpretResult {
    InterpretOK,
    _InterpretCompileErr,
    _InterpretRunTimeErr,
}

impl VM {
    pub fn new_vm(chunk: Chunk) -> Self {
        Self {
            chunk: chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
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

                i if i == OpCode::OpNegate as u8 => {
                    let val = -self.stack.pop().unwrap();
                    self.stack.push(val);
                    continue;
                }

                i if i == OpCode::OpAdd as u8 => {
                    self.binary_op(|a, b| a + b);
                    continue;
                }

                i if i == OpCode::OpSubtract as u8 => {
                    self.binary_op(|a, b| a - b);
                    continue;
                }

                i if i == OpCode::OpDivide as u8 => {
                    self.binary_op(|a, b| a / b);
                    continue;
                }

                i if i == OpCode::OpMultiply as u8 => {
                    self.binary_op(|a, b| a * b);
                    continue;
                }

                _ => todo!(),
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

    fn binary_op(&mut self, op: impl Fn(f64, f64) -> f64) {
        let b = self.stack.pop().unwrap();
        let c = self.stack.pop().unwrap();

        match (c, b) {
            (Values::Double(x), Values::Double(y)) => {
                self.stack.push(Values::Double(op(x, y)));
            }
        }
    }
}
