use std::{
    fs,
    io::{self, Read},
};

use crate::{chunk::Chunk, vm::VM};

mod chunk;
mod compiler;
mod scanner;
mod table;
mod vm;

fn main() -> io::Result<()> {
    let mut file = String::new();
    fs::File::open("/home/mohammed/programming/Rust/practice/numen/example.num")?
        .read_to_string(&mut file)?;

    let chunk = Chunk::new_chunk();
    chunk.disassembler("BYTECODE");
    VM::new_vm(chunk).interpret(&file);

    Ok(())
}

pub fn cli() -> io::Result<()> {
    Ok(())
}
