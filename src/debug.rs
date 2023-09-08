use crate::chunk::{Chunk, OpCode};

#[allow(dead_code)]
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: u8 = 0;

    while offset < chunk.code.len() as u8{
        offset = disassemble_instruction(chunk, offset);
    }
}


pub fn disassemble_instruction(chunk: &Chunk, offset: u8) -> u8 {
    print!("{} ", offset);

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

fn jump_instruction(name: &str, chunk: &Chunk, sign: i8, offset: u8) -> u8{
    let jump: usize = (usize::from(chunk.code[offset as usize + 1]) << 8) + usize::from(chunk.code[offset as usize + 2]);

    print!("{} {} -> {}", name, offset, (offset as usize) + 3 + (sign as usize) * jump);
    offset + 3
}

fn constant_instruction(name: &str, chunk: &Chunk, offset: u8) -> u8 {
    let constant = chunk.code[offset as usize + 1];
    print!("{} {} ", name, constant);
    chunk.constants[constant as usize].print_value();
    println!();
    offset + 2

}

fn simple_instruction(name: &str, offset: u8) -> u8 {
    println!("{}", name);
    offset + 1
}

fn byte_instruction(name: &str, chunk: &Chunk, offset: u8) -> u8 {
    let slot = chunk.code[offset as usize + 1];
    println!("{} {}", name, slot);
    offset + 2
}