use crate::error::*;
use crate::expr::*;
use crate::literal::Literal;
use crate::stmt::*;
use crate::token::*;
use crate::token_type::*;

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
    had_error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &[Token]) -> Parser {
        Parser {
            tokens,
            current: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    pub fn success(&self) -> bool {
        !self.had_error
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        let result = if self.is_match(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };

        if result.is_err() {
            self.synchronize();
        }

        result
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.is_match(&[TokenType::Print]) {
            return self.print_statement();
        } else if self.is_match(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(BlockStmt {
                statements: self.block()?,
            }));
        } else {
            return self.expression_statement();
        }
    }

    pub fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after value.")?;
        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.is_match(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::SemiColon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    pub fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after value.")?;
        Ok(Stmt::Expression(ExpressionStmt { expression: value }))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(statements)
    }

    // assignment → IDENTIFIER "=" assignment | equality ;
    pub fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.equality()?;

        if self.is_match(&[TokenType::Equal]) {
            let equals = self.previous().dup();
            let value = self.assignment()?;

            if let Expr::Variable(expr) = expr {
                return Ok(Expr::Assign(AssignExpr {
                    name: expr.name.dup(),
                    value: Box::new(value),
                }));
            }

            self.error(&equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr: Expr = self.comparison()?;

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator: Token = self.previous().dup();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr: Expr = self.term()?;

        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.previous().dup();
            let right: Expr = self.term()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr: Expr = self.factor()?;

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous().dup();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr: Expr = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous().dup();
            let right: Expr = self.unary()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous().dup();
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Literal::Bool(false)),
            }));
        }
        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Literal::Bool(true)),
            }));
        }
        if self.is_match(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Literal::Nil),
            }));
        }
        if self.is_match(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal.clone(),
            }));
        }
        if self.is_match(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous().dup(),
            }));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr: Expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }

        Err(LoxError::error(0, "failed primary parser"))
    }

    fn is_match(&mut self, ttypes: &[TokenType]) -> bool {
        for &t in ttypes {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(ttype) {
            Ok(self.advance().dup())
        } else {
            Err(self.error(&self.peek().dup(), message))
        }
    }

    fn error(&mut self, token: &Token, message: &str) -> LoxError {
        self.had_error = true;
        LoxError::parse_error(token, message)
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().is(TokenType::SemiColon) {
                return;
            }
            if matches!(
                self.peek().token_type(),
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }
            self.advance();
        }
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().is(ttype)
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().is(TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}
