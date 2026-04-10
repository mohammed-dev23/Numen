use crate::{
    chunk::{Chunk, OpCode, Values},
    vm::VM,
};

mod chunk;
mod debug;
mod vm;

fn main() {
    let mut chunk = Chunk::new_chunk();

    let value1 = chunk.add_constant(Values::Double(1.0));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value1 as u8, 123);

    let value2 = chunk.add_constant(Values::Double(1.0));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value2 as u8, 123);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);

    let value3 = chunk.add_constant(Values::Double(9.0));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value3 as u8, 123);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);

    let value4 = chunk.add_constant(Values::Double(12.0));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value4 as u8, 123);

    chunk.write_chunk(OpCode::OpSubtract as u8, 123);

    chunk.write_chunk(OpCode::OpR as u8, 123);
    chunk.disassembler("TEST RUN: BASIC MATH".to_string());
    VM::new_vm(chunk).interpret();
}

pub fn cli() -> anyhow::Result<()> {
    Ok(())
}
