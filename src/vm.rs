use crate::chunk::{Chunk, OpCode, Values};

pub struct VM {
    chunk: Chunk,
    ip: usize,
}

pub enum InterpretResult {
    InterpretOK,
    InterpretCompileErr,
    InterpretRunTimeErr,
}

impl VM {
    pub fn new_vm(chunk: Chunk) -> Self {
        Self {
            chunk: chunk,
            ip: 0,
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_bytes();
            match instruction {
                i if i == OpCode::OpR as u8 => return InterpretResult::InterpretOK,
                i if i == OpCode::OpC as u8 => {
                    let constant = self.read_constant();
                    println!(" {:?} ", constant);
                    break InterpretResult::InterpretOK;
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
}
