use std::collections::HashMap;

use crate::ast::Value;
use crate::error::RoxError;
use crate::token::Token;
use crate::token::TokenType;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: &Token) -> Result<Value, RoxError> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone());
        } else if self.enclosing.as_ref().is_some() {
            self.enclosing.as_mut().unwrap().get(&name)
        } else {
            Err(RoxError::UndefinedVariableError(name.clone()))
        }
    }

    pub fn assign(&mut self, name: Token, value: Value) {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Literal;

    #[test]
    fn test_environment() {
        let mut env = Environment::new(None);
        let token = Token::new(TokenType::Number, "a", Some(Literal::Number(5.0)), 1);
        env.define("a".to_string(), Value::Number(5.0));
        let res = env.get(&token);
        assert_eq!(env.get(&token).unwrap(), Value::Number(5.0));
    }

    #[test]
    fn test_enclosing_environment() {
        let mut enclosing_env = Environment::new(None);
        enclosing_env.define("a".to_string(), Value::Number(5.0));
        let mut env = Environment::new(Some(Box::new(enclosing_env.clone())));
        let token = Token::new(TokenType::Number, "a", Some(Literal::Number(5.0)), 1);

        assert_eq!(env.get(&token).unwrap(), Value::Number(5.0));
    }
}
