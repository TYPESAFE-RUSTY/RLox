mod interpreter;
mod parser;
mod scanner;
use interpreter::Interpreter;
use parser::Parser;

use crate::scanner::*;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

fn run(source: String) -> Vec<parser::stmt::Stmt> {
    let mut scanner: Scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();
    let mut parser = Parser::new(tokens.to_vec());
    parser.parse()
}

fn run_file(path: &String) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    match fs::read_to_string(path) {
        Err(msg) => Err(msg.to_string()),
        Ok(source) => Ok(interpreter.interpret(run(source))),
    }
}

fn run_prompt() -> () {
    let mut interpreter = Interpreter::new();

    loop {
        let mut line = String::new();
        print!("> ");
        let _ = io::stdout().flush(); //this is needed as rust stores print data to line buffer and > is not printed before the buffer is full
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if line.trim() == "exit()" {
                    return;
                }
            }
            Err(msg) => {
                println!("Error : {}", msg);
            }
        };
        interpreter.interpret(run(line))
    }
}

fn _error(line: u32, message: String) -> () {
    _report(line, "", message)
}

fn _report(line: u32, location: &str, message: String) -> () {
    println!("[line {}] Error {} : {}", line, location, message);
}

fn main() {
    let args: Vec<String> = env::args().collect(); //This contains location of the rlox.exe as first argument
    if args.len() > 2 {
        // more arguments are provided
        println!("Usage: rlox [script]");
        process::exit(64)
    }
    if args.len() == 2 {
        // file path is provided as first argument
        match run_file(&args[1]) {
            Ok(_) => process::exit(0),
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        }
    } else {
        // no argument provided
        run_prompt();
    }
}
