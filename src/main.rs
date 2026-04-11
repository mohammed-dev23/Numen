use crate::{
    chunk::{Chunk, OpCode, Values},
    vm::VM,
};

mod chunk;
mod debug;
mod vm;

fn main() {
    let mut chunk = Chunk::new_chunk();

    let value1 = chunk.add_constant(Values::Int(1));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value1 as u8, 123);

    let value2 = chunk.add_constant(Values::Int(1));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value2 as u8, 123);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);

    let value3 = chunk.add_constant(Values::Int(9));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value3 as u8, 123);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);

    let value4 = chunk.add_constant(Values::Int(12));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value4 as u8, 123);

    chunk.write_chunk(OpCode::OpSubtract as u8, 123);

    let value5 = chunk.add_constant(Values::Float(8.5));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value5 as u8, 123);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);

    let value6 = chunk.add_constant(Values::Int(19));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value6 as u8, 123);

    chunk.write_chunk(OpCode::OpMultiply as u8, 123);

    let value7 = chunk.add_constant(Values::Int(5));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value7 as u8, 123);

    chunk.write_chunk(OpCode::OpMod as u8, 123);

    let value8 = chunk.add_constant(Values::Float(9.7));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value8 as u8, 123);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);

    let value9_1 = chunk.add_constant(Values::Int(7));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value9_1 as u8, 123);

    let value9_2 = chunk.add_constant(Values::Int(2));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value9_2 as u8, 123);

    chunk.write_chunk(OpCode::OpPow as u8, 123);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);

    let value10 = chunk.add_constant(Values::Int(10));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value10 as u8, 123);
    chunk.write_chunk(OpCode::OpSqrt as u8, 123);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);

    let value11 = chunk.add_constant(Values::Float(9.9));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value11 as u8, 123);
    chunk.write_chunk(OpCode::OpNegate as u8, 123);

    chunk.write_chunk(OpCode::OpMultiply as u8, 123);

    chunk.write_chunk(OpCode::OpAbs as u8, 123);

    chunk.write_chunk(OpCode::OpFloor as u8, 123);

    let value12 = chunk.add_constant(Values::Float(1.8));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value12 as u8, 123);

    chunk.write_chunk(OpCode::OpAdd as u8, 123);

    chunk.write_chunk(OpCode::OpCeil as u8, 123);

    chunk.write_chunk(OpCode::OpSin as u8, 123);

    let value13 = chunk.add_constant(Values::Float(900.0));
    chunk.write_chunk(OpCode::OpC as u8, 123);
    chunk.write_chunk(value13 as u8, 123);

    //OpEq works and gives false
    //OpNotEq works and gives true
    //OpGt works and gives true
    //OpLs works and gives false
    //OpLte works and gives false
    //OpGte works and gives true
    chunk.write_chunk(OpCode::OpGte as u8, 123);

    chunk.write_chunk(OpCode::OpR as u8, 123);
    chunk.disassembler("TEST RUN: BASIC MATH".to_string());
    VM::new_vm(chunk).interpret();
}

pub fn cli() {}
