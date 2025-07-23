use crate::{chunk::{Chunk, OpCode}, debug};
use log::debug as log_debug;

#[allow(dead_code)]
// given a chunk, print all instructions in the chunk
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    log_debug!("== {} ==", name);
    
    // offset is used to keep track of the current position in the bytecode instructions
    let mut offset: u8 = 0;  // TODO should this be a bigger type?

    // instructions can have different lengths so get next offset from disassemble_instruction
    while offset < chunk.code.len() as u8{
        offset = disassemble_instruction(chunk, offset);
    }
}

// disassemble_instruction takes a chunk and an offset, and prints the instruction at that offset
pub fn disassemble_instruction(chunk: &Chunk, offset: u8) -> u8 {
    let mut debug_string = "".to_string();
    debug_string.push_str(&format!("{:03} ", offset));

    // if offset is not the first instruction, print the line number
    // TODO: equality check seems weird
    if offset > 0 && chunk.get_line(offset as usize) == chunk.get_line(offset as usize) {
        debug_string.push_str("| ");
    } else {
        debug_string.push_str(&chunk.get_line(offset as usize).to_string());
        debug_string.push_str(" ");
    }
    
    let instruction = chunk.code[offset as usize];
    let code: OpCode = instruction.into();
    match code {
        OpCode::OpReturn => return simple_instruction("OP_RETURN", offset, debug_string),
        OpCode::OpConstant => return constant_instruction("OP_CONSTANT", chunk, offset, debug_string),
        OpCode::OpNegate => return simple_instruction("OP_NEGATE", offset, debug_string),
        OpCode::OpAdd => return simple_instruction("OP_ADD", offset, debug_string),
        OpCode::OpSubtract => return simple_instruction("OP_SUBTRACT", offset, debug_string),
        OpCode::OpMultiply => return simple_instruction("OP_MULTIPLY", offset, debug_string),
        OpCode::OpDivide => return simple_instruction("OP_DIVIDE", offset, debug_string),
        OpCode::OpNot => return simple_instruction("OP_NOT", offset, debug_string),
        OpCode::OpVoid => return simple_instruction("OP_VOID", offset, debug_string),
        OpCode::OpTrue => return simple_instruction("OP_TRUE", offset, debug_string),
        OpCode::OpFalse => return simple_instruction("OP_FALSE", offset, debug_string),
        OpCode::OpEqual => return simple_instruction("OP_EQUAL", offset, debug_string),
        OpCode::OpGreater => return simple_instruction("OP_GREATER", offset, debug_string),
        OpCode::OpLess => return simple_instruction("OP_LESS", offset, debug_string),
        OpCode::OpEmit => return simple_instruction("OP_EMIT", offset, debug_string),
        OpCode::OpPop => return simple_instruction("OP_POP", offset, debug_string),
        OpCode::OpDefineGlobal => return constant_instruction("OP_DEFINE_GLOBAL", chunk, offset, debug_string),
        OpCode::OpGetGlobal => return constant_instruction("OP_GET_GLOBAL", chunk, offset, debug_string),
        OpCode::OpSetGlobal => return constant_instruction("OP_SET_GLOBAL", chunk, offset, debug_string),
        OpCode::OpGetLocal => return byte_instruction("OP_GET_LOCAL", chunk, offset, debug_string),
        OpCode::OpSetLocal => return byte_instruction("OP_SET_LOCAL", chunk, offset, debug_string),
        OpCode::OpJump => return jump_instruction("OP_JUMP", chunk, 1, offset, debug_string),
        OpCode::OpJumpIfFalse => return jump_instruction("OP_JUMP_IF_FALSE", chunk, 1, offset, debug_string),
        OpCode::OpLoop => return jump_instruction("OP_LOOP", chunk, -1, offset, debug_string)
    }
}

fn jump_instruction(name: &str, chunk: &Chunk, sign: i16, offset: u8, mut debug_string: String) -> u8{
    let jump = (usize::from(chunk.code[offset as usize + 1]) << 8) | usize::from(chunk.code[offset as usize + 2]);

    let jump_to = if sign > 0 {
        offset as usize + 3 + jump
    } else {
        offset as usize + 3 - jump
    };

    debug_string.push_str(&format!("{} {} -> {}", name, jump, jump_to));
    log_debug!("{}", debug_string);

    offset + 3
}

/*
    name: name of the instruction
    chunk: the chunk containing the instruction
    offset: the offset of the instruction in the chunk
*/
fn constant_instruction(name: &str, chunk: &Chunk, offset: u8, mut debug_string: String) -> u8 {
    let constant_index = chunk.code[offset as usize + 1]; // constant is stored after the opcode
    //log_debug!("constant instruction {} {}", name, constant_index); // this is constant index
    let constant_value = chunk.constants[constant_index as usize].read_value_str();

    debug_string.push_str(&format!("{} {} {}", name, constant_index, constant_value));
    log_debug!("{}", debug_string);
    
    offset + 2
}

/*
    Print name of the instruction and increment offset by 1
*/
fn simple_instruction(name: &str, offset: u8, mut debug_string: String) -> u8 {
    debug_string.push_str(name);
    log_debug!("{}", debug_string);

    offset + 1
}

fn byte_instruction(name: &str, chunk: &Chunk, offset: u8, mut debug_string: String) -> u8 {
    let slot = chunk.code[offset as usize + 1];
    debug_string.push_str(&format!("{} {}", name, slot));
    log_debug!("{}", debug_string);

    offset + 2
}