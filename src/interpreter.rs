use crate::ast::{Expr, ExprVisitor, Stmt, StmtVisitor, Value};
use crate::error::RoxError;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType::{
    Bang, BangEqual, EqualEqual, Greater, GreaterEqual, Less, LessEqual, Minus, Plus, Slash, Star,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    fn execute(&mut self, stmt: &Stmt) {}

    pub fn interpret(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    fn stringify(&mut self, value: Value) -> String {
        match value {
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => f64::to_string(&n),
            Value::String_(s) => s.clone(),
            Value::Nil => "nil".to_owned(),
        }
    }
}

impl StmtVisitor<Value> for Interpreter {
    fn visit_expr_stmt(&mut self, stmt_expr: Expr) {
        self.evaluate(stmt_expr);
    }

    fn visit_print_stmt(&mut self, stmt_expr: Expr) {
        let value = self.evaluate(stmt_expr);
        println!("{:?}", value);
    }
}

impl ExprVisitor<Value> for Interpreter {
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
    use crate::token::Literal;

    #[test]
    fn test_interpret_literal_boolean() {}

    #[test]
    fn test_interpret_print_statement() {
        let mut interpreter = Interpreter::new();
        let statements = vec![ast::Stmt::Print(ast::Expr::Literal(Literal::String_(
            "one".to_string(),
        )))];

        let res = interpreter.interpret(&statements);
    }
}
