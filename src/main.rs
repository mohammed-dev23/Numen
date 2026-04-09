use crate::{
    chunk::{Chunk, OpCode, Values},
    vm::VM,
};

mod chunk;
mod parser;
mod vm;

fn main() {
    let mut chunk = Chunk::new_chunk();
    let constant_idx = chunk.add_constant(Values::Double(8.9));
    chunk.write_chunk(OpCode::OpC as u8, 124);
    chunk.write_chunk(constant_idx as u8, 124);
    chunk.write_chunk(OpCode::OpR as u8, 125);
    chunk.disassembler("TEST RUN: BASIC MATH".to_string());
    VM::new_vm(chunk).interpret();
}

pub fn cli() -> anyhow::Result<()> {
    Ok(())
}
