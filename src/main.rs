mod token_type;
use ast_printer::AstPrinter;

mod ast_printer;
mod error;
mod expr;
mod generate_ast;
mod interpreter;
mod literal;
mod scanner;
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
            m.report("");
            std::process::exit(65);
        }
    }
    Ok(())
}
fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    stdout().flush();

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(_) => {
                    // ignore, already reported
                }
            }
        } else {
            break;
        }
        print!("> ");
        stdout().flush();
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    // for token in tokens {
    //     println!("{:?}", token);
    // }
    // Ok(())

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        None => {}
        Some(expr) => {
            let printer = AstPrinter {};
            println!("AST Printer:\n{}", printer.print(&expr)?);
        }
    }
    Ok(())
}
