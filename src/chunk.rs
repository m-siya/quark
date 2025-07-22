/*
    a chunk is a sequence of bytecode instructions. 

    one instruction has a one byte operation code (opcode)

*/
//use crate::value;
use crate::value::Value;

// OpCode is an enum that represents the different operation codes
// each OpCode corresponds to a specific operation that the virtual machine can perform
#[derive(Clone, Copy, Debug)]
pub enum OpCode { 
    OpConstant, // load a constant value onto the stack
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
    OpVoid,
    OpTrue,
    OpFalse,
    OpNot,
    OpEqual,
    OpGreater,
    OpLess,
    OpEmit,
    OpPop,
    OpDefineGlobal,
    OpGetGlobal,
    OpSetGlobal,
    OpGetLocal,
    OpSetLocal,
    OpJumpIfFalse,
    OpJump,
    OpLoop,
}

// implement From trait for OpCode to convert OpCode to u8 and vice versa
impl From<OpCode> for u8 {
    fn from(code: OpCode) -> u8 {
        code as u8
    }
}

impl From<u8> for OpCode {
    fn from(index: u8) -> OpCode {
        match index {
            0 => OpCode::OpConstant,
            1 => OpCode::OpAdd,
            2 => OpCode::OpSubtract,
            3 => OpCode::OpMultiply,
            4 => OpCode::OpDivide,
            5 => OpCode::OpNegate,
            6 => OpCode::OpReturn,
            7 => OpCode::OpVoid,
            8 => OpCode::OpTrue,
            9 => OpCode::OpFalse,
            10 => OpCode::OpNot,
            11 => OpCode::OpEqual,
            12 => OpCode::OpGreater,
            13 => OpCode::OpLess,
            14 => OpCode::OpEmit,
            15 => OpCode::OpPop,
            16 => OpCode::OpDefineGlobal,
            17 => OpCode::OpGetGlobal,
            18 => OpCode::OpSetGlobal,
            19 => OpCode::OpGetLocal,
            20 => OpCode::OpSetLocal,
            21 => OpCode::OpJumpIfFalse,
            22 => OpCode::OpJump,
            23 => OpCode::OpLoop,
            _ => panic!("Error. Invalid OpCode code")
        }
    }
}

// access the chunk's capacity and count using vector's .capacity() and .len()
#[derive(Debug)]
// Chunk is a struct that represents a chunk of bytecode
// it contains the bytecode instructions, constants, and line numbers
pub struct Chunk {
    // TODO: should code be a Vec<u8> or a Vec<OpCode>?
    pub code: Vec<u8>, // sequence of OpCodes stored as u8
    pub constants: Vec<Value>,
    pub lines: Vec<i32>,
}

impl Chunk {
    pub fn new() -> Chunk{
        Chunk {code: Vec::new(), constants: Vec::new(), lines: Vec::new()}
    }

    // pub fn write(&mut self, byte: u8, line: i32) {
    //     self.code.push(byte);
    //     self.lines.push(line);
    // }

    /*
        compressed form of write line.

        takes a chunk, a byte (the opcode converted to u8), and a line number
    */
    pub fn write(&mut self, byte: u8, line: i32) {

        self.code.push(byte);

        let len = self.lines.len();

        if len == 0 {
            self.lines.push(line);
            self.lines.push(1);
            return;
        }

        let mut on_same_line = false;

        if *self.lines.get(len - 2).unwrap() == line{
                on_same_line = true;        
        }

        match on_same_line {
            true => {
                *self.lines.last_mut().unwrap() += 1;
            }
            false => {
                self.lines.push(line);
                self.lines.push(1);
            }
        }
    }

    pub fn get_line(&self, instruction_index: usize) -> i32 {
        let mut current_index = 0;
        let mut current_line = 0;

        for chunk in self.lines.chunks(2){
            let line = chunk[0];
            let run_length = chunk[1];

            let next_index = current_index + run_length ;

            if (instruction_index as i32) < next_index {
                current_line = line;
                break;
            }

            current_index = next_index;
        }

        current_line

    }
    // add_constant returns usize
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> &Value {
        &self.constants[index]
    }

}