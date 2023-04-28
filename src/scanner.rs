use std::collections::HashMap;

use crate::error::LoxError;
use crate::literal::Literal;
use crate::token::*;
use crate::token_type::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: {
                let mut m = HashMap::new();
                m.insert("and".to_string(), TokenType::And);
                m.insert("class".to_string(), TokenType::Class);
                m.insert("else".to_string(), TokenType::Else);
                m.insert("false".to_string(), TokenType::False);
                m.insert("for".to_string(), TokenType::For);
                m.insert("fun".to_string(), TokenType::Fun);
                m.insert("if".to_string(), TokenType::If);
                m.insert("nil".to_string(), TokenType::Nil);
                m.insert("or".to_string(), TokenType::Or);
                m.insert("print".to_string(), TokenType::Print);
                m.insert("return".to_string(), TokenType::Return);
                m.insert("super".to_string(), TokenType::Super);
                m.insert("this".to_string(), TokenType::This);
                m.insert("true".to_string(), TokenType::True);
                m.insert("var".to_string(), TokenType::Var);
                m.insert("while".to_string(), TokenType::While);
                m
            },
        }
    }
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(e) => {
                    e.report("");
                    had_error = Some(e);
                }
            }
        }

        self.tokens.push(Token::eof(self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.match_char('/') {
                    // this means comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.scan_string();
            }
            _ => {
                if self.is_dight(c) {
                    self.scan_number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(LoxError::error(self.line, "Unexpected character."));
                }
            }
        }
        Ok(())
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = self.source[self.start..self.current].to_string();
        let token_type = (*self.keywords.get(&text).unwrap_or(&TokenType::Identifier)).clone();
        self.add_token(token_type);
    }

    fn scan_number(&mut self) {
        while self.is_dight(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_dight(self.peek_next()) {
            // consume the "."
            self.advance();

            while self.is_dight(self.peek()) {
                self.advance();
            }
        }
        let number = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.add_token_object(TokenType::Number, Some(Literal::Number(number)));
        // self.add_token(TokenType::Number);
    }

    fn scan_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            LoxError::error(self.line, "Unterminated string.");
            return;
        }

        // the closing "
        self.advance();

        // value: String = self.source.chars().skip(self.start + 1).take(self.current - self.start + 2).collect();
        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_object(TokenType::String, Some(Literal::String(value)));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_dight(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_dight(c)
    }

    fn advance(&mut self) -> char {
        let res = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        res
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_object(token_type, None);
    }
    fn add_token_object(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }
}
