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

    fn visit_binary_expr(&self, _expr: &BinaryExpr) -> Result<Literal, LoxError> {
        Ok(Literal::Nil)
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
