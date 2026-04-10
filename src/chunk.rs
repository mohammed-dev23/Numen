use std::{ops::Neg, u8};

//this reper macro tells rust to treat or eunm values as bytes
#[repr(u8)]
pub enum OpCode {
    OpR,
    OpC,
    OpNegate,
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
}

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constant: ValueArray,
    pub line: Vec<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum Values {
    #[allow(warnings)]
    Double(f64),
}

impl Neg for Values {
    type Output = Values;

    fn neg(self) -> Self::Output {
        match self {
            Values::Double(d) => Values::Double(-d),
        }
    }
}

#[derive(Debug)]
pub struct ValueArray {
    pub values: Vec<Values>,
}

impl ValueArray {
    pub fn new_value() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write_value(&mut self, values: Values) {
        self.values.push(values);
    }
}

impl Chunk {
    pub fn new_chunk() -> Self {
        Self {
            code: Vec::new(),
            constant: ValueArray::new_value(),
            line: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, bytes: u8, line: usize) {
        self.code.push(bytes);
        self.line.push(line);
    }

    pub fn add_constant(&mut self, value: Values) -> usize {
        self.constant.write_value(value);
        self.constant.values.len() - 1
    }

    pub fn disassembler(&self, chunk: String) {
        println!("=={}==", chunk);
        let mut offset = 0;

        while offset < self.code.len() {
            offset = self.disassembler_instruction(offset);
        }
    }
    pub fn disassembler_instruction(&self, offset: usize) -> usize {
        print!(" {:04} ", offset);

        if offset > 0 && self.line[offset] == self.line[offset - 1] {
            print!("  | ")
        } else {
            print!(" {:4} ", self.line[offset])
        }

        let instruction = self.code[offset];
        match instruction {
            i if i == OpCode::OpR as u8 => Self::simple_instruction(offset, "OPR".to_string()),
            i if i == OpCode::OpC as u8 => {
                Self::constant_instruction(&self, "OPC".to_string(), offset)
            }
            i if i == OpCode::OpAdd as u8 => Self::simple_instruction(offset, "OPADD".to_string()),
            i if i == OpCode::OpDivide as u8 => {
                Self::simple_instruction(offset, "OPDIVIDE".to_string())
            }
            i if i == OpCode::OpSubtract as u8 => {
                Self::simple_instruction(offset, "OPSUBTRACT".to_string())
            }
            i if i == OpCode::OpMultiply as u8 => {
                Self::simple_instruction(offset, "OPMULTIPLY".to_string())
            }
            _ => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }

    fn simple_instruction(offset: usize, chunk: String) -> usize {
        println!(" {} ", chunk);
        offset + 1
    }

    fn constant_instruction(&self, name: String, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        println!(
            "{:<16} {:4} '{:?}'",
            name, constant, self.constant.values[constant as usize]
        );
        offset + 2
    }
}
