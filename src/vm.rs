use core::panic;

#[cfg(feature = "trace")]
use trace::trace;
#[cfg(feature = "trace")]
trace::init_depth_var!();

use std::collections::HashMap;
use std::hash::Hash;
use std::string;

use crate::debug;
use crate::chunk::{Chunk, OpCode};
use crate::object::{Object, ObjString};
use crate::value::Value;
use crate::compiler::Compiler;
//use crate::compiler::Compiler;

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

//to produce formated runtime error messages
// takes string and a variable number of arguments.
macro_rules! run_time_error {
    ($chunk: expr, $ip: expr, $format: expr $(, $($arg:expr), *)?) => {
        {
            eprintln!($format $(, $($arg), *)?);
            let line = $chunk.lines[$ip - 1];
            eprintln!("[line {}] in script", line);
            //self.reset_stack();
        }
    };
}


#[derive(Debug)]
pub struct VM{
    //chunk: Chunk,
    ip: usize, //indexes into the next instruction in the chunk
    stack: Vec<Value>,
    heap: Vec<Object>,
    globals: HashMap<String, Value>
}

impl VM {
    pub fn new() -> VM{
        VM {ip: 0, stack: Vec::new(), heap: Vec::new(), globals: HashMap::new()}
    }

    pub fn reset_stack(&mut self) {
        self.stack = Vec::new();
    }

    //returns the next instruction to which ip points to
    fn read_byte(&mut self, chunk: &Chunk) -> OpCode{
        let instruction: OpCode = chunk.code[self.ip].into();
        self.ip += 1;
        instruction
        
    }

    fn read_constant<'a>(&'a mut self, chunk: &'a Chunk) -> &Value {
        let index: usize = chunk.code[self.ip] as usize;
        self.ip += 1;
       // chunk.constants[index]
       chunk.get_constant(index)
    }

    fn read_string(&mut self, chunk: &Chunk) -> String {
        let string_object = self.read_constant(chunk);
        match string_object.get_inner_string() {
            Some(inner_string) => inner_string.to_string(),
            None => panic!("Empy string as identifier"),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        let value: Option<Value> = self.stack.pop();
        
        match value {
            Some(value) => return value,
            None => panic!("VM stack is empty"),
        }
    }

    fn peek(&self, depth: usize) -> &Value {
        &self.stack[self.stack.len() - depth - 1]
    }
    
    fn concatenate(&mut self) {
        let op_r = self.pop();
        let op_l = self.pop();

        match (op_r, op_l) {
            (Value::ValObject(object_right), Value::ValObject(object_left)) => {
                let result = format!("{}{}", object_left.get_object_data().unwrap_or(""), object_right.get_object_data().unwrap_or(""));
                self.push(Value::ValObject(Object::ObjString(ObjString::from_str(&result))));
            },
            (_, _) => {
                panic!("Operands must be strings to concatenate");
            }
        }
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut chunk: Chunk = Chunk::new();
        let mut compiler = Compiler::new(&mut chunk, source);
        
        if !compiler.compile(){
            return InterpretResult::CompileError;
        }

       // compiler.compile(source);

        self.ip = 0;
        let result: InterpretResult = self.run(&chunk);
        result
        
    }

    fn run(&mut self, chunk: &Chunk) -> InterpretResult {
        macro_rules! binary_op {
            ($op:tt) => {
                {
                    let op_r = self.peek(0);
                    //println!("this works");
                    let op_l = self.peek(1);
                    
                    match (op_r.is_number(), op_l.is_number()) {
                        (true, true) => {
                            let right_operand: Value= self.pop();
                            let left_operand: Value = self.pop();
                            self.push(Value::from(left_operand $op right_operand));
                        }
                        (_, _) => {
                            run_time_error!(chunk, self.ip, "Error: {}", "Operands must be numbers");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
            }
        }
        
        loop {

            //only for debugging, put under flag later
            print!("          ");
            for slot in self.stack.iter() {
                print!("[ ");
                (slot).print_value();
                print!(" ]");
            }
            println!();

            debug::disassemble_instruction(chunk, self.ip as u8);
            
            // debug code ends  

            let instruction: OpCode = self.read_byte(chunk);

            match instruction {
                OpCode::OpReturn => {
                    // print!("Final value on stack when program returns: ");
                    // (self.pop()).print_value();
                    // println!();

                    return InterpretResult::Ok;
                },

                OpCode::OpEmit => {
                    (self.pop()).print_value();
                    println!();
                },

                OpCode::OpConstant => {
                    let constant = self.read_constant(chunk).clone();
                    self.push(constant);
                    //println!("{}", constant);  
                }
                OpCode::OpAdd => {
                    let op_r = self.peek(0);
                    let op_l = self.peek(1);

                    match (op_r, op_l) {
                        (Value::ValObject(Object::ObjString(_)), Value::ValObject(Object::ObjString(_))) => self.concatenate(),
                        (Value::ValNumber(_), Value::ValNumber(_)) => binary_op!(+),
                        (_, _) => {
                            run_time_error!(chunk, self.ip, "Error: {}", "Operands must be numbers or strings");
                            return InterpretResult::RuntimeError;
                        },
                    }
                },
                OpCode::OpSubtract => binary_op!(-),
                OpCode::OpMultiply => binary_op!(*),
                OpCode::OpDivide => binary_op!(/),
                OpCode::OpNot => {
                    let value = self.peek(0);
                    match value.is_bool() {
                        true => {
                            let top_val = self.pop();
                            self.push(!top_val);
                        }
                        false => {
                            run_time_error!(&chunk, self.ip, "Error: {}", "Operand must be a boolean");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::OpNegate => {
                    let value = self.peek(0);
                    match value.is_number() {
                        true => {
                            let top_val = self.pop();
                            self.push(-top_val);
                        }

                        false => {
                            run_time_error!(&chunk, self.ip, "Error: {}", "Operand must be a number");
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::OpVoid => self.push(Value::ValVoid(())),
                OpCode::OpTrue => self.push(Value::ValBool(true)),
                OpCode::OpFalse => self.push(Value::ValBool(false)),
                OpCode::OpPop => {
                    self.pop();
                },
                OpCode::OpGetLocal => {
                    let slot = self.read_byte(chunk);
                    print!("{:?}", &self.stack[slot as u8 as usize]);
                    self.stack.push(self.stack[slot as u8 as usize].clone())
                },
                OpCode::OpSetLocal => {
                    let slot = self.read_byte(chunk);
                    self.stack[slot as u8 as usize] = self.peek(0).clone();
                }
                OpCode::OpGetGlobal => {
                    //println!("hiii");
                    let name = self.read_string(chunk);
                    //println!("hiii");

                    match self.globals.get(&name){
                        Some(value) => {
                            self.push(value.clone());
                            //println!("{:?}", value);
                        }
                        None => {
                            run_time_error!(&chunk, self.ip, "Error: {}", "Undefined variable ".to_owned() + &name);
                            return InterpretResult::RuntimeError;
                        }
                    }
                }
                OpCode::OpDefineGlobal => {
                    let name = self.read_string(chunk);
                    self.globals.insert(name, self.peek(0).clone());
                    self.pop();

                },
                OpCode::OpSetGlobal => {
                    let name = self.read_string(chunk);

                    match self.globals.get(&name){
                        None => {
                            run_time_error!(&chunk, self.ip, "Error: {}", "Undefined variable ".to_owned() + &name);
                            return InterpretResult::RuntimeError;
                        }
                        Some(_) => {
                            println!("{:?}", self.stack);
                            self.globals.insert(name, self.peek(0).clone());
                        }
                    }
                },
                OpCode::OpEqual => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Value::ValBool(a == b));
                }
                OpCode::OpGreater => binary_op!(>),
                OpCode::OpLess => binary_op!(<),
            }
        }        
    }
}