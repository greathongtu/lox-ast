use std::fmt;
use std::ops::*;
use std::cmp::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    ArithmeticError,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Nil => write!(f, "nil"),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::ArithmeticError => panic!("Should not be trying to print ArithmeticError"),
        }
    }
}

impl Sub for Literal {
    type Output = Literal;

    fn sub(self, other: Self) -> Literal {
        match (self, other) {
            (Literal::Number(left), Literal::Number(right)) => Literal::Number(left - right),
            _ => Literal::ArithmeticError,
        }
    }
}

impl Div for Literal {
    type Output = Literal;

    fn div(self, other: Self) -> Literal {
        match (self, other) {
            (Literal::Number(left), Literal::Number(right)) => Literal::Number(left / right),
            _ => Literal::ArithmeticError,
        }
    }
}

impl Mul for Literal {
    type Output = Literal;

    fn mul(self, other: Self) -> Literal {
        match (self, other) {
            (Literal::Number(left), Literal::Number(right)) => Literal::Number(left * right),
            _ => Literal::ArithmeticError,
        }
    }
}
impl Add for Literal {
    type Output = Literal;

    fn add(self, other: Self) -> Literal {
        match (self, other) {
            (Literal::Number(left), Literal::Number(right)) => Literal::Number(left + right),
            (Literal::String(left), Literal::String(right)) => Literal::String(format!("{}{}", left, right)),
            _ => Literal::ArithmeticError,
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Literal::Nil, o) => {
                if o == &Literal::Nil {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }

            (Literal::Number(left), Literal::Number(right)) => left.partial_cmp(right),
            _ => None,
        }
    }
}