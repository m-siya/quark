mod chunk;
mod debug;
mod value;
mod vm;
mod scanner;
mod compiler;
mod object;

//use crate::chunk::{Chunk, OpCode};
//use crate::value::Value;
use crate::vm::{VM, InterpretResult};

use std::io::Write;
use std::{env, io};

fn main() {
    let mut vm: VM = VM::new();

    let args: Vec<String> = env::args().collect();
    let argc = args.len() - 1;

    match argc {
        1 => {
            repl(&mut vm);
        },
        2 => {
            run_file(&mut vm, &args[2]);
                     
        },
        _ => {
            println!("Incorrect arguments.");
            println!("Usage: quark [path]");
            std::process::exit(64);
        },
    }
}

fn repl(vm: &mut VM) {
    loop {
        print!("> ");

        std::io::stdout().flush().unwrap();

        let mut line = String::new();

        match io::stdin().read_line(&mut line).unwrap() {
            0 => {
                println!();
                break;
            },
            _ => {
                vm.interpret(&line);
            },
        }

    }
}

fn run_file(vm: &mut VM, path: &str) -> io::Result<()> {
    let buffer = std::fs::read_to_string(path)?;
    match vm.interpret(&buffer) {
        InterpretResult::CompileError => std::process::exit(65),
        InterpretResult::RuntimeError => std::process::exit(70),
        InterpretResult::Ok => std::process::exit(0),
    }
}