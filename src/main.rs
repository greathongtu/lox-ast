mod token_type;

mod token;
mod scanner;
mod error;

use scanner::*;
use error::*;
use std::{
    env::args,
    io::{self, BufRead},
};
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Usage: lox-ast [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]).expect("Could not run file");
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => {}
        Err(m) => {
            m.report("".to_string());
            std::process::exit(65);
        }
    }
    Ok(())
}
fn run_prompt() {
    let stdin = io::stdin();
    loop {
        for line in stdin.lock().lines() {
            println!("> ");
            let mut line = String::new();
            match io::stdin().read_line(&mut line) {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("error: {}", error);
                    break;
                }
            }
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(error) => {
                    error.report("".to_string());    
                }
            }
        }
    }
}



fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

