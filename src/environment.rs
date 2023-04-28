use std::collections::HashMap;

use crate::error::*;
use crate::literal;
use crate::literal::*;
use crate::token::*;
use crate::Lox;

pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: &str, value: Literal) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Literal, LoxError> {
        if let Some(literal) = self.values.get(name.as_string()) {
            Ok(literal.clone())
        } else {
            Err(LoxError::runtime_error(
                &name,
                &format!("Undefined variable '{}'.", name.as_string()),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_type::*;

    #[test]
    fn can_define_a_variable() {
        let mut e = Environment::new();

        e.define("One", Literal::Bool(true));

        assert!(e.values.contains_key("One"));
        assert_eq!(
            e.values.get("One").unwrap(),
            &Literal::Bool(true)
        );
    }

    #[test]
    fn can_redefine_a_variable() {
        let mut e = Environment::new();
        e.define("Two", Literal::Bool(true));
        e.define("Two", Literal::Number(12.0));
        assert_eq!(
            e.values.get("Two").unwrap(),
            &Literal::Number(12.0)
        );
    }

    #[test]
    fn can_look_up_a_variable() {
        let mut e = Environment::new();
        e.define("Three", Literal::String("foo".to_string()));

        let three_tok = Token::new(TokenType::Identifier, "Three".to_string(), None, 0);
        assert_eq!(e.get(&three_tok).unwrap(), Literal::String("foo".to_string()));
    }

    #[test]
    fn error_when_variable_undefined() {
        let e = Environment::new();
        let three_tok = Token::new(TokenType::Identifier, "Three".to_string(), None, 0);
        assert!(e.get(&three_tok).is_err());
    }
}

