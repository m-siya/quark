use crate::chunk::{Chunk, OpCode};

#[allow(dead_code)]
// given a chunk, print all instructions in the chunk
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    
    // offset is used to keep track of the current position in the bytecode instructions
    let mut offset: u8 = 0;  // TODO should this be a bigger type?

    // instructions can have different lengths so get next offset from disassemble_instruction
    while offset < chunk.code.len() as u8{
        offset = disassemble_instruction(chunk, offset);
    }
}

// disassemble_instruction takes a chunk and an offset, and prints the instruction at that offset
pub fn disassemble_instruction(chunk: &Chunk, offset: u8) -> u8 {
    print!("{} ", offset); // print position in the chunk

    // if offset is not the first instruction, print the line number
    // TODO: equality check seems weird
    if offset > 0 && chunk.get_line(offset as usize) == chunk.get_line(offset as usize) {
        print!("  | ");
    } else {
        print!("{} ", chunk.get_line(offset as usize));
    }
    
    let instruction = chunk.code[offset as usize];
    let code: OpCode = instruction.into();
    match code {
        OpCode::OpReturn => return simple_instruction("OP_RETURN", offset),
        OpCode::OpConstant => return constant_instruction("OP_CONSTANT", chunk, offset),
        OpCode::OpNegate => return simple_instruction("OP_NEGATE", offset),
        OpCode::OpAdd => return simple_instruction("OP_ADD", offset),
        OpCode::OpSubtract => return simple_instruction("OP_SUBTRACT", offset),
        OpCode::OpMultiply => return simple_instruction("OP_MULTIPLY", offset),
        OpCode::OpDivide => return simple_instruction("OP_DIVIDE", offset),
        OpCode::OpNot => return simple_instruction("OP_NOT", offset),
        OpCode::OpVoid => return simple_instruction("OP_VOID", offset),
        OpCode::OpTrue => return simple_instruction("OP_TRUE", offset),
        OpCode::OpFalse => return simple_instruction("OP_FALSE", offset),
        OpCode::OpEqual => return simple_instruction("OP_EQUAL", offset),
        OpCode::OpGreater => return simple_instruction("OP_GREATER", offset),
        OpCode::OpLess => return simple_instruction("OP_LESS", offset),
        OpCode::OpEmit => return simple_instruction("OP_EMIT", offset),
        OpCode::OpPop => return simple_instruction("OP_POP", offset),
        OpCode::OpDefineGlobal => return constant_instruction("OP_DEFINE_GLOBAL", chunk, offset),
        OpCode::OpGetGlobal => return constant_instruction("OP_GET_GLOBAL", chunk, offset),
        OpCode::OpSetGlobal => return constant_instruction("OP_SET_GLOBAL", chunk, offset),
        OpCode::OpGetLocal => return byte_instruction("OP_GET_LOCAL", chunk, offset),
        OpCode::OpSetLocal => return byte_instruction("OP_SET_LOCAL", chunk, offset),
        OpCode::OpJump => return jump_instruction("OP_JUMP", chunk, 1, offset),
        OpCode::OpJumpIfFalse => return jump_instruction("OP_JUMP_IF_FALSE", chunk, 1, offset),
        OpCode::OpLoop => return jump_instruction("OP_LOOP", chunk, -1, offset)
    }
}

fn jump_instruction(name: &str, chunk: &Chunk, sign: i16, offset: u8) -> u8{
    let jump = (usize::from(chunk.code[offset as usize + 1]) << 8) | usize::from(chunk.code[offset as usize + 2]);

    let jump_to = if sign > 0 {
        offset as usize + 3 + jump
    } else {
        offset as usize + 3 - jump
    };

    print!("{} {} -> {}", name, offset, jump_to);
    offset + 3
}

/*
  
*/
fn constant_instruction(name: &str, chunk: &Chunk, offset: u8) -> u8 {
    let constant_index = chunk.code[offset as usize + 1]; // constant is stored after the opcode
    print!("{} {}", name, constant_index); // this is constant index
    chunk.constants[constant_index as usize].print_value();
    println!();
    offset + 2

}

/*
    Print name of the instruction and increment offset by 1
*/
fn simple_instruction(name: &str, offset: u8) -> u8 {
    println!("{}", name);
    offset + 1
}

fn byte_instruction(name: &str, chunk: &Chunk, offset: u8) -> u8 {
    let slot = chunk.code[offset as usize + 1];
    println!("{} {}", name, slot);
    offset + 2
}