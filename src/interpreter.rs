use crate::ast::{Expr, ExprVisitor, Stmt, StmtVisitor, Value};
use crate::environment::Environment;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType::{
    Bang, BangEqual, EqualEqual, Greater, GreaterEqual, Identifier, Less, LessEqual, Minus, Plus,
    Slash, Star, Var,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(None),
        }
    }

    fn execute(&mut self, stmt: &Stmt) {}

    fn execute_block(&mut self, statements: Vec<Stmt>, environment: Environment) {
        let previous: Environment = environment;
        for statement in statements {
            self.execute(&statement);
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.execute(statement);
        }
    }
}

impl StmtVisitor<Value> for Interpreter {
    fn visit_var_stmt(&mut self, token: Token, stmt_expr: Option<Expr>) {
        let value: Value;

        value = if let Some(stmt_expr) = stmt_expr {
            self.evaluate(stmt_expr)
        } else {
            Value::Nil
        };

        self.environment.define(token.lexeme, value);
    }

    fn visit_expr_stmt(&mut self, stmt_expr: Expr) {
        self.evaluate(stmt_expr);
    }

    fn visit_print_stmt(&mut self, stmt_expr: Expr) {
        let value = self.evaluate(stmt_expr);
        println!("{:?}", value);
    }

    fn visit_block_stmt(&mut self, statements: Vec<Stmt>) {
        let value = self.execute_block(statements, Environment::new(None));
        println!("{:?}", value);
    }
}

impl ExprVisitor<Value> for Interpreter {
    fn visit_assignment_expr(&mut self, name: Token, expr: Box<Expr>) -> Value {
        let value = self.evaluate(*expr);

        self.environment.assign(name, value.clone());
        value
    }

    fn visit_var_expr(&mut self, name: Token) -> Value {
        self.environment.get(&name).unwrap()
    }

    fn visit_literal_expr(&mut self, literal: Literal) -> Value {
        literal.into()
    }

    fn visit_grouping_expr(&mut self, group: Box<Expr>) -> Value {
        self.evaluate(*group)
    }

    fn visit_unary_expr(&mut self, op: Token, right: Box<Expr>) -> Value {
        let right = self.evaluate(*right);

        match op.token_type {
            Minus => {
                if let Value::Number(n) = right {
                    return Value::Number(-n);
                } else {
                    panic!("{:?} must be a number", right);
                }
            }
            Bang => {
                return Value::Bool(!self.is_truthy(right));
            }

            _ => Value::Nil,
        }
    }

    fn visit_binary_expr(&mut self, left: Box<Expr>, op: Token, right: Box<Expr>) -> Value {
        let left = self.evaluate(*left);
        let right = self.evaluate(*right);

        match op.token_type {
            Minus => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Number(l - r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            Slash => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Number(l / r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            Star => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Number(l * r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            Plus => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Number(l + r)
                } else if let (Value::String_(l), Value::String_(r)) = (left.clone(), right.clone())
                {
                    Value::String_(l + &r)
                } else {
                    panic!(
                        "{:?} and {:?} must both be numbers or both be strings",
                        left, right
                    );
                }
            }
            Greater => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Bool(l > r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            GreaterEqual => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Bool(l >= r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            Less => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Bool(l < r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            LessEqual => {
                if let (Value::Number(l), Value::Number(r)) = (left.clone(), right.clone()) {
                    Value::Bool(l <= r)
                } else {
                    panic!("{:?} and {:?} must be numbers", left, right);
                }
            }
            BangEqual => Value::Bool(!self.is_equal(left, right)),
            EqualEqual => Value::Bool(self.is_equal(left, right)),
            _ => Value::Nil,
        }
    }

    fn is_truthy(&mut self, value: Value) -> bool {
        if value == Value::Nil || value == Value::Bool(false) {
            return false;
        }

        true
    }

    fn is_equal(&mut self, a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String_(a), Value::String_(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;
    use crate::error::RoxError;
    use crate::token::Literal;

    #[test]
    fn test_interpret_print_statement() -> Result<(), RoxError> {
        let mut interpreter = Interpreter::new();
        let statements = vec![ast::Stmt::Print(ast::Expr::Literal(Literal::String_(
            "one".to_string(),
        )))];

        let res = interpreter.interpret(&statements);
        println!("{:?}", res);
        Ok(())
    }

    #[test]
    fn test_interpret_var_statement() -> Result<(), RoxError> {
        let mut interpreter = Interpreter::new();
        let statements = vec![ast::Stmt::Var(
            Token::new(Var, "a", None, 1),
            Some(ast::Expr::Literal(Literal::String_("one".to_string()))),
        )];
        let res = interpreter.interpret(&statements);
        Ok(())
    }
}
