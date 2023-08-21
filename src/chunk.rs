//use crate::value;
use crate::value::Value;

#[derive(Clone, Copy)]
pub enum OpCode {
   
    OpConstant,
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
}

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
            _ => panic!("Error. Invalid OpCode code")
        }
    }
}

// access the chunk's capacity and count using vector's .capacity() and .len()
//#[derive(Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
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

    //compressed form of write line
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
    // add_constant returns u8 
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn get_constant(&self, index: usize) -> &Value {
        &self.constants[index]
    }

}