use std::{
    fs,
    io::{self, Read},
};

use crate::{chunk_values::Chunk, vm::VM};

mod chunk_values;
mod compiler;
mod debug;
mod scanner;
mod table;
mod vm;

fn main() -> io::Result<()> {
    let mut file = String::new();
    fs::File::open("/home/mohammed/programming/Rust/practice/numen/example.num")?
        .read_to_string(&mut file)?;

    let chunk = Chunk::new_chunk();

    chunk.disassembler("");

    VM::new_vm(chunk).interpret(&file);

    Ok(())
}
