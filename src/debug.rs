use super::{
    Chunk,
    chunk_values::{OpCode, ValueArray, Values},
};

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

    pub fn disassembler(&self, chunk: &str) {
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
                Self::constant_instruction(self, "OPC".to_string(), offset)
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
            i if i == OpCode::OpMod as u8 => Self::simple_instruction(offset, "OPMOD".to_string()),
            i if i == OpCode::OpPow as u8 => Self::simple_instruction(offset, "OPPOW".to_string()),
            i if i == OpCode::OpNegate as u8 => {
                Self::simple_instruction(offset, "OPNEGATE".to_string())
            }
            i if i == OpCode::OpSqrt as u8 => {
                Self::simple_instruction(offset, "OPSQRT".to_string())
            }
            i if i == OpCode::OpAbs as u8 => Self::simple_instruction(offset, "OPABS".to_string()),
            i if i == OpCode::OpFloor as u8 => {
                Self::simple_instruction(offset, "OPFLOOR".to_string())
            }
            i if i == OpCode::OpCeil as u8 => {
                Self::simple_instruction(offset, "OPCEIL".to_string())
            }
            i if i == OpCode::OpSin as u8 => Self::simple_instruction(offset, "OPSAN".to_string()),
            i if i == OpCode::OpCos as u8 => Self::simple_instruction(offset, "OPCOS".to_string()),
            i if i == OpCode::OpTan as u8 => Self::simple_instruction(offset, "OPTAN".to_string()),
            i if i == OpCode::OpEqEq as u8 => {
                Self::simple_instruction(offset, "OPEQEQ".to_string())
            }
            i if i == OpCode::OpNotEq as u8 => {
                Self::simple_instruction(offset, "OPNOTEQ".to_string())
            }
            i if i == OpCode::OpGt as u8 => Self::simple_instruction(offset, "OPGT".to_string()),
            i if i == OpCode::OpLt as u8 => Self::simple_instruction(offset, "OPLT".to_string()),
            i if i == OpCode::OpGte as u8 => Self::simple_instruction(offset, "OPGTE".to_string()),
            i if i == OpCode::OpLte as u8 => Self::simple_instruction(offset, "OPLTE".to_string()),
            i if i == OpCode::OpDivideDivide as u8 => {
                Self::simple_instruction(offset, "OPDIVDIV".to_string())
            }
            i if i == OpCode::OpNot as u8 => Self::simple_instruction(offset, "OPNOT".to_string()),
            i if i == OpCode::OpEq as u8 => Self::simple_instruction(offset, "OPEQ".to_string()),
            i if i == OpCode::OpPrint as u8 => {
                Self::simple_instruction(offset, "OPPRINT".to_string())
            }
            i if i == OpCode::OpPop as u8 => Self::simple_instruction(offset, "OPPOP".to_string()),
            i if i == OpCode::OpDefGlobal as u8 => {
                Self::constant_instruction(self, "OPDEFGLOBAL:".to_string(), offset)
            }
            i if i == OpCode::OpGetGlobal as u8 => {
                Self::constant_instruction(self, "OPGETGLOBAL".to_string(), offset)
            }
            i if i == OpCode::OpSetGlobal as u8 => {
                Self::constant_instruction(self, "OPSETGLOBAL".to_string(), offset)
            }
            i if i == OpCode::OpSetLocal as u8 => {
                Self::byte_instruction(self, "OPSETLOCAL".to_string(), offset)
            }
            i if i == OpCode::OpGetLocal as u8 => {
                Self::byte_instruction(self, "OPGETLOCAL".to_string(), offset)
            }
            i if i == OpCode::OpDefFixed as u8 => {
                Self::constant_instruction(self, "OPDEFFIXED".to_string(), offset)
            }
            i if i == OpCode::OpAdd as u8 => Self::simple_instruction(offset, "OPADD".to_string()),
            i if i == OpCode::OpOr as u8 => Self::simple_instruction(offset, "OPOR".to_string()),
            i if i == OpCode::OpSetLocalFixed as u8 => {
                Self::byte_instruction(self, "OPSETLOCALFiXED".to_string(), offset)
            }
            i if i == OpCode::OpJumpIfFalse as u8 => {
                Self::jump_instruction(&self, "OPJUMPIFFALSE".to_string(), 1, offset)
            }
            i if i == OpCode::OpJump as u8 => {
                Self::jump_instruction(&self, "OPJUMP".to_string(), 1, offset)
            }

            i if i == OpCode::OpLoop as u8 => {
                Self::jump_instruction(&self, "OPLOOP".to_string(), 1, offset)
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

    fn byte_instruction(&self, name: String, offset: usize) -> usize {
        let slot = self.code[offset + 1];
        println!("{:<16} {:4}", name, slot);
        offset + 2
    }

    fn jump_instruction(&self, name: String, sign: i64, offset: usize) -> usize {
        let jump = ((self.code[offset + 1]) as u16) << 8 | (self.code[offset + 2] as u16);

        println!(
            "{:<16} {:4} -> {}",
            name,
            offset,
            offset as i64 + 3 + sign * jump as i64
        );

        offset + 3
    }
}
