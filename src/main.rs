mod chunk;
mod debug;
mod value;

use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

fn main() {
    let mut chunk: Chunk = Chunk::new();


    let constant: usize = chunk.add_constant(Value::ValNumber(4.0));
    chunk.write(OpCode::OpConstant as u8, 123);
    chunk.write(constant as u8, 123);
    chunk.write(OpCode::OpReturn as u8, 123);

    debug::disassemble_chunk(&chunk, "test_chunk");

}
