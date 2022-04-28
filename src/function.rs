use crate::ast::Stmt;
use crate::ast::Value;
use crate::environment::Environment;
use crate::error::RoxError;
use crate::token::Token;
use crate::Interpreter;

#[derive(Clone)]
pub enum RoxFunction {
    Native {
        arity: usize,
        body: Box<fn(&Vec<Value>) -> Value>,
    },
    User {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
}

impl RoxFunction {
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &Vec<Value>,
    ) -> Result<Value, RoxError> {
        match self {
            RoxFunction::Native { body, .. } => Ok(body(arguments)),
            RoxFunction::User {
                name, params, body, ..
            } => {
                let mut environment = Environment::new(Some(Box::new(interpreter.globals.clone())));
                for i in 0..params.len() {
                    environment.define(params[i].lexeme.clone(), arguments[i].clone());
                }

                interpreter.execute_block(body.clone(), environment);

                Ok(Value::Nil)
            }
        }
    }
}
