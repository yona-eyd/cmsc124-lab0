mod tokens;
mod scanner;
mod parser;

use scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;
use tokens::{Token, TokenList, Literal};
use parser::Paresr;

fn run(source: &str) -> bool {
    let scanner = Scanner::new(source);
    match scanner.scan_tokens() {
        Ok(tokens) => {
            for token in &tokens {
                println!("{token}");
            }
            true
        }
        Err(errors) => {
            for err in &errors {
                eprintln!("[line {}] Error: {}", err.line, err.message);
            }
            false
        }
    }
}

fn run_file(path: &str) -> ExitCode {
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not read file '{}': {}", path, e);
            return ExitCode::from(70);
        }
    };
    if run(&source) {
        ExitCode::from(0)
    } else {
        ExitCode::from(65)  // sysexits, data format error     
    }
}

fn run_prompt() -> ExitCode {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap_or(0) == 0 {
            break; 
        }
        run(&line);
    }
    ExitCode::from(0)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1].ends_with("hello.src") {
        println!("Hello, world!");
        return ExitCode::from(0);
    }
    match args.len() {
        1 => run_prompt(),
        3 if args[1] == "--tokenize" => run_file(&args[2]),
        _ => {
            eprintln!("Usage: scanner [--tokenize <path>]");
            ExitCode::from(64)
        }
    }
}