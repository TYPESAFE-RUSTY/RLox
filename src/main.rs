mod scanner;
mod token;
use crate::scanner::*;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

fn run(source: String) -> () {
    let mut scanner = Scanner::new(source);
    let _tokens = scanner.scan_tokens();
}

fn run_file(path: &String) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(source) => return Ok(run(source)),
    }
}

fn run_prompt() -> () {
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
        run(line.trim().to_string())
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
    } else if args.len() == 2 {
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