use std::cell::RefCell;

use crate::environment::Environment;
use crate::error::*;
use crate::expr::*;
use crate::literal::*;
use crate::stmt::*;
use crate::token_type::*;

pub struct Interpreter {
    // RefCell because we want to mutate the environment
    environment: RefCell<Environment>,
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{value}");
        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxError> {
        let value = if let Some(initializer) = &stmt.initializer {
            self.evaluate(&initializer)?
        } else {
            Literal::Nil
        };
        self.environment
            .borrow_mut()
            .define(&stmt.name.as_string(), value);
        Ok(())
    }
}

// interpreter is a visitor of expressions, an operation
impl ExprVisitor<Literal> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Literal, LoxError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Literal, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Literal, LoxError> {
        let left: Literal = self.evaluate(&expr.left)?;
        let right: Literal = self.evaluate(&expr.right)?;
        let op = expr.operator.token_type();

        let result = match (left, right) {
            (Literal::Number(left), Literal::Number(right)) => match op {
                TokenType::Minus => Literal::Number(left - right),
                TokenType::Slash => Literal::Number(left / right),
                TokenType::Star => Literal::Number(left * right),
                TokenType::Plus => Literal::Number(left + right),
                TokenType::Greater => Literal::Bool(left > right),
                TokenType::GreaterEqual => Literal::Bool(left >= right),
                TokenType::Less => Literal::Bool(left < right),
                TokenType::LessEqual => Literal::Bool(left <= right),
                TokenType::BangEqual => Literal::Bool(left != right),
                TokenType::EqualEqual => Literal::Bool(left == right),
                _ => {
                    todo!("need to work on your code dude");
                }
            },
            (Literal::Number(left), Literal::String(right)) => match op {
                TokenType::Plus => Literal::String(format!("{left}{right}")),
                _ => Literal::ArithmeticError,
            },
            (Literal::String(left), Literal::Number(right)) => match op {
                TokenType::Plus => Literal::String(format!("{left}{right}")),
                _ => Literal::ArithmeticError,
            },
            (Literal::String(left), Literal::String(right)) => match op {
                TokenType::Plus => Literal::String(format!("{left}{right}")),
                TokenType::BangEqual => Literal::Bool(left != right),
                TokenType::EqualEqual => Literal::Bool(left == right),
                _ => Literal::ArithmeticError,
            },
            (Literal::Bool(left), Literal::Bool(right)) => match op {
                TokenType::BangEqual => Literal::Bool(left != right),
                TokenType::EqualEqual => Literal::Bool(left == right),
                _ => Literal::ArithmeticError,
            },
            (Literal::Nil, Literal::Nil) => match op {
                TokenType::BangEqual => Literal::Bool(false),
                TokenType::EqualEqual => Literal::Bool(true),
                _ => Literal::ArithmeticError,
            },
            (Literal::Nil, _) => match op {
                TokenType::EqualEqual => Literal::Bool(false),
                TokenType::BangEqual => Literal::Bool(true),
                _ => Literal::ArithmeticError,
            },
            _ => Literal::ArithmeticError,
        };

        if result == Literal::ArithmeticError {
            Err(LoxError::runtime_error(
                &expr.operator,
                "Illegal expression",
            ))
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

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Literal, LoxError> {
        return self.environment.borrow().get(&expr.name);
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: RefCell::new(Environment::new()),
        }
    }
    fn evaluate(&self, expr: &Expr) -> Result<Literal, LoxError> {
        expr.accept(self)
    }

    fn execute(&self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    // Lox follows Ruby’s simple rule: false and nil are falsey, and everything else is truthy.··
    fn is_truthy(&self, literal: &Literal) -> bool {
        !matches!(literal, Literal::Nil | Literal::Bool(false))
    }

    pub fn interpret(&self, statements: &[Stmt]) -> bool {
        let mut success = true;
        for statement in statements {
            if let Err(e) = self.execute(statement) {
                e.report("");
                success = false;
                break;
            }
        }
        success
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
        let terp = Interpreter::new();
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
        let terp = Interpreter::new();
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
        let terp = Interpreter::new();
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
        let terp = Interpreter::new();
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
        let terp = Interpreter::new();
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
        let terp = Interpreter::new();
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
        let terp = Interpreter::new();
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
        let terp = Interpreter::new();
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
        let terp = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal(Literal::Number(15.0)),
            operator: Token::new(TokenType::Greater, ">".to_string(), None, 123),
            right: make_literal(Literal::Bool(true)),
        };
        let result = terp.visit_binary_expr(&binary_expr);
        assert!(result.is_err());
    }

    fn run_comparison_test(tok: &Token, cmps: Vec<bool>) {
        let nums = vec![14.0, 15.0, 16.0];
        let terp = Interpreter::new();

        for (c, nums) in cmps.iter().zip(nums) {
            let binary_expr = BinaryExpr {
                left: make_literal(Literal::Number(nums)),
                operator: tok.dup(),
                right: make_literal(Literal::Number(15.0)),
            };
            let result = terp.visit_binary_expr(&binary_expr);
            assert!(result.is_ok());
            assert_eq!(
                result.ok(),
                Some(Literal::Bool(*c)),
                "Testing {} {} 15.0",
                nums,
                tok.as_string()
            );
        }
    }

    #[test]
    fn test_less_than() {
        run_comparison_test(
            &Token::new(TokenType::Less, "<".to_string(), None, 123),
            vec![true, false, false],
        );
    }

    #[test]
    fn test_less_than_or_equal_to() {
        run_comparison_test(
            &Token::new(TokenType::LessEqual, "<=".to_string(), None, 123),
            vec![true, true, false],
        );
    }

    #[test]
    fn test_greater_than() {
        run_comparison_test(
            &Token::new(TokenType::Greater, ">".to_string(), None, 123),
            vec![false, false, true],
        );
    }

    #[test]
    fn test_greater_than_or_equal_to() {
        run_comparison_test(
            &Token::new(TokenType::GreaterEqual, ">=".to_string(), None, 123),
            vec![false, true, true],
        );
    }

    #[test]
    fn test_equals_nums() {
        run_comparison_test(
            &Token::new(TokenType::EqualEqual, "==".to_string(), None, 123),
            vec![false, true, false],
        );
    }

    #[test]
    fn test_not_equals_nums() {
        run_comparison_test(
            &Token::new(TokenType::BangEqual, "!=".to_string(), None, 123),
            vec![true, false, true],
        );
    }

    #[test]
    fn test_not_equals_string() {
        let terp = Interpreter::new();
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
        let terp = Interpreter::new();
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
        let terp = Interpreter::new();
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
    fn test_var_stmt_defined() {
        let terp = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 123);
        let var_stmt = VarStmt {
            name: name.dup(),
            initializer: Some(*make_literal(Literal::Number(23.0))),
        };
        assert!(terp.visit_var_stmt(&var_stmt).is_ok());
        assert_eq!(
            terp.environment.borrow().get(&name).unwrap(),
            Literal::Number(23.0)
        );
    }

    #[test]
    fn test_var_stmt_undefined() {
        let terp = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 123);
        let var_stmt = VarStmt {
            name: name.dup(),
            initializer: None,
        };
        assert!(terp.visit_var_stmt(&var_stmt).is_ok());
        assert_eq!(terp.environment.borrow().get(&name).unwrap(), Literal::Nil);
    }

    #[test]
    fn test_variable_expr() {
        let terp = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 123);
        let var_stmt = VarStmt {
            name: name.dup(),
            initializer: Some(*make_literal(Literal::Number(23.0))),
        };
        assert!(terp.visit_var_stmt(&var_stmt).is_ok());

        let var_expr = VariableExpr { name: name.dup() };
        assert_eq!(
            terp.visit_variable_expr(&var_expr).unwrap(),
            Literal::Number(23.0)
        );
    }

    #[test]
    fn test_undefined_variable_expr() {
        let terp = Interpreter::new();
        let name = Token::new(TokenType::Identifier, "foo".to_string(), None, 123);
        let var_expr = VariableExpr { name: name.dup() };
        assert!(terp.visit_variable_expr(&var_expr).is_err());
    }
}
