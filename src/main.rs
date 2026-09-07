mod tokens;
mod scanner;

use scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let scanned = scanner.scan_tokens().clone();
    for token in &scanned {
        println!("{}", token.to_string());
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
    run(&source);
    ExitCode::from(0) 
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
    match args.len() {
        1 => run_prompt(),
        3 if args[1] == "--tokenize" => run_file(&args[2]),
        _ => {
            eprintln!("Usage: scanner [--tokenize <path>]");
            ExitCode::from(64)
        }
    }
}