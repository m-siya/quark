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
use log::{debug};


fn main() {
    let mut vm: VM = VM::new();
    
    let args: Vec<String> = env::args().collect();
    let argc = args.len() - 1;

    match argc {
        1 => {
            repl(&mut vm);
        },
        2 | 3 => {
            if argc == 3 && args[3] == "--debug" {
                env::set_var("RUST_LOG", "debug");
                env_logger::init();
            }
            run_file(&mut vm, &args[2]);
        }
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

fn run_file(vm: &mut VM, path: &str) {
    let buffer = std::fs::read_to_string(path).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", path, err);
        std::process::exit(74);
    });
    match vm.interpret(&buffer) {
        InterpretResult::CompileError => {
            eprintln!("Compile error in file: {}", path);
            std::process::exit(65);
        }
        InterpretResult::RuntimeError => {
            eprintln!("Runtime error in file: {}", path);
            std::process::exit(70);
        }
        InterpretResult::Ok => {
            println!("Execution completed successfully.");
            std::process::exit(0);
        }
    }
}