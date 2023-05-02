mod token_type;

// mod ast_printer;
mod environment;
mod error;
mod expr;
mod interpreter;
use interpreter::*;
mod literal;
mod scanner;
mod stmt;
mod token;

mod parser;
use crate::parser::*;

use error::*;
use scanner::*;
use std::{
    env::args,
    io::{self, stdout, BufRead, Write},
};
fn main() {
    // // test pretty printing
    // let expression = Expr::Binary(BinaryExpr {
    //     left: Box::new(Expr::Unary(UnaryExpr {
    //         operator: Token {
    //             token_type: TokenType::Minus,
    //             lexeme: "-".to_string(),
    //             literal: None,
    //             line: 1,
    //         },
    //         right: Box::new(Expr::Literal(LiteralExpr {
    //             value: Some(Literal::Number(123.0)),
    //         })),
    //     })),
    //     operator: Token {
    //         token_type: TokenType::Star,
    //         lexeme: "*".to_string(),
    //         literal: None,
    //         line: 1,
    //     },
    //     right: Box::new(Expr::Grouping(GroupingExpr {
    //         expression: Box::new(Expr::Literal(LiteralExpr {
    //             value: Some(Literal::Number(45.67)),
    //         })),
    //     })),
    // });
    // let printer = ast_printer::AstPrinter;
    // println!("{}", printer.print(&expression).unwrap());
    // return;

    let args: Vec<String> = args().collect();
    let lox = Lox::new();
    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]).expect("Could not run file"),
        _ => {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }
}

struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    pub fn new() -> Lox {
        Lox {
            interpreter: Interpreter::new(),
        }
    }

    fn run_file(&self, path: &str) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        if self.run(buf).is_err() {
            // Ignore: error was already reported
            std::process::exit(65);
        }
        Ok(())
    }
    fn run_prompt(&self) {
        let stdin = io::stdin();
        print!("> ");
        let _ = stdout().flush();

        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                match self.run(line) {
                    Ok(_) => {}
                    Err(_) => {
                        // ignore, already reported
                    }
                }
            } else {
                break;
            }
            print!("> ");
            let _ = stdout().flush();
        }
    }

    fn run(&self, source: String) -> Result<(), LoxResult> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;
        if parser.success() {
            self.interpreter.interpret(&statements);
        }
        Ok(())
    }
}
