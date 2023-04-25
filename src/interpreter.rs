use crate::error::*;
use crate::expr::*;
use crate::literal::*;
use crate::token_type::*;

pub struct Interpreter {}

impl ExprVisitor<Literal> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Literal, LoxError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Literal, LoxError> {
        Ok(self.evaluate(&expr.expression)?)
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Literal, LoxError> {
        let left: Literal = self.evaluate(&expr.left)?;
        let right: Literal = self.evaluate(&expr.right)?;

        let result = match expr.operator.token_type() {
            TokenType::Minus => left - right,
            TokenType::Slash => left / right,
            TokenType::Star => left * right,
            TokenType::Plus => left + right,
            TokenType::Greater => Literal::Bool(left > right),
            TokenType::GreaterEqual => Literal::Bool(left >= right),
            TokenType::Less => Literal::Bool(left < right),
            TokenType::LessEqual => Literal::Bool(left <= right),
            TokenType::BangEqual => Literal::Bool(left != right),
            TokenType::EqualEqual => Literal::Bool(left == right),

            _ => {
                todo!("need to work on your code dude");
            }
        };

        if result == Literal::ArithmeticError {
            Err(LoxError::error(expr.operator.line, "Illegal expression"))
        } else {
            Ok(result)
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Literal, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type() {
            TokenType::Minus => match right {
                Literal::Number(n) => return Ok(Literal::Number(-n)),
                _ => return Ok(Literal::Nil),
            },
            TokenType::Bang => {
                if self.is_truthy(&right) {
                    Ok(Literal::Bool(false))
                } else {
                    Ok(Literal::Bool(true))
                }
            }
            _ => Err(LoxError::error(0, "Unreachable according to Nystrom")),
        }
    }
}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Result<Literal, LoxError> {
        expr.accept(self)
    }

    // Lox follows Ruby’s simple rule: false and nil are falsey, and everything else is truthy.··
    fn is_truthy(&self, literal: &Literal) -> bool {
        !matches!(literal, Literal::Nil | Literal::Bool(false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;

    fn make_literal(o: Literal) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr { value: Some(o) }))
    }

    fn make_literal_string(s: &str) -> Box<Expr> {
        make_literal(Literal::String(s.to_string()))
    }

    #[test]
    fn test_unary_minus() {
        let terp = Interpreter {};
        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 123),
            right: make_literal(Literal::Number(123.0)),
        };
        let result = terp.visit_unary_expr(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(-123.0)));
    }

    #[test]
    fn test_unary_not() {
        let terp = Interpreter {};
        let unary_expr = UnaryExpr {
            operator: Token::new(TokenType::Bang, "!".to_string(), None, 123),
            right: make_literal(Literal::Bool(false)),
        };
        let result = terp.visit_unary_expr(&unary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_subtraction() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(15.0)),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 123),
            right: make_literal(Literal::Number(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(8.0)));
    }

    #[test]
    fn test_multiplication() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(15.0)),
            operator: Token::new(TokenType::Star, "*".to_string(), None, 123),
            right: make_literal(Literal::Number(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(105.0)));
    }

    #[test]
    fn test_division() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(21.0)),
            operator: Token::new(TokenType::Slash, "/".to_string(), None, 123),
            right: make_literal(Literal::Number(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(3.0)));
    }

    #[test]
    fn test_addition() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(21.0)),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 123),
            right: make_literal(Literal::Number(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Number(28.0)));
    }

    #[test]
    fn test_string_concatination() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal_string("hello, "),
            operator: Token::new(TokenType::Plus, "+".to_string(), None, 123),
            right: make_literal_string("world!"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(
            result.ok(),
            Some(Literal::String("hello, world!".to_string()))
        );
    }

    #[test]
    fn test_arithmetic_error_for_subtration() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(15.0)),
            operator: Token::new(TokenType::Minus, "-".to_string(), None, 123),
            right: make_literal(Literal::Bool(true)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_err());
    }

    #[test]
    fn test_arithmetic_error_for_greater() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(15.0)),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 123),
            right: make_literal(Literal::Bool(true)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_greater_than_equal_to() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(15.0)),
            operator: Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 123),
            right: make_literal(Literal::Number(15.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_greater_than_true() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(15.0)),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 123),
            right: make_literal(Literal::Number(7.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_greater_than_false() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(15.0)),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 123),
            right: make_literal(Literal::Number(17.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(false)));
    }

    #[test]
    fn test_equals() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(15.0)),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 123),
            right: make_literal(Literal::Number(15.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_not_equals_string() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal_string("hello"),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 123),
            right: make_literal_string("hellx"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(false)));
    }

    #[test]
    fn test_equals_string() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal_string("world"),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 123),
            right: make_literal_string("world"),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_equals_nil() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Nil),
            operator: Token::new(TokenType::EqualEqual, "==".to_string(), None, 123),
            right: make_literal(Literal::Nil),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }

    #[test]
    fn test_not_equals() {
        let terp = Interpreter {};
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(15.0)),
            operator: Token::new(TokenType::BangEqual, "!=".to_string(), None, 123),
            right: make_literal(Literal::Number(16.0)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Literal::Bool(true)));
    }
}
