use std::cmp::*;
use std::fmt;

use crate::callable::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nil,
    Func(Callable),
    Bool(bool),
    Number(f64),
    String(String),
    ArithmeticError,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Func(_) => write!(f, "<func>"),
            Literal::Nil => write!(f, "nil"),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::ArithmeticError => panic!("Should not be trying to print ArithmeticError"),
        }
    }
}
