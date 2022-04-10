use crate::ast::{Expr, ExprVisitor, Value};
use crate::error::RoxError;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType::{
    Bang, BangEqual, EqualEqual, Greater, GreaterEqual, Less, LessEqual, Minus, Plus, Slash, Star,
};

struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    fn interpret(&mut self, expression: Expr) -> String {
        let value: Value = self.evaluate(expression);

        println!("{:?}", value.clone());
        println!("{:?}", self.stringify(value.clone()));
        self.stringify(value)
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

    #[test]
    fn test_interpret_literal_boolean() {
        let mut interpreter = Interpreter::new();
        let expression = Expr::Literal(Literal::Bool(true));
        let res = interpreter.interpret(expression);
        assert_eq!(res, "true");

        let expression = Expr::Literal(Literal::Bool(false));
        let res = interpreter.interpret(expression);
        assert_eq!(res, "false");
    }

    #[test]
    fn test_interpret_literal_string() {
        let mut interpreter = Interpreter::new();
        let expression = Expr::Literal(Literal::String_("Hello, world!".to_owned()));
        let res = interpreter.interpret(expression);
        assert_eq!(res, "Hello, world!");
    }

    #[test]
    fn test_interpret_literal_number() {
        let mut interpreter = Interpreter::new();
        let float = 5.0;
        let expression = Expr::Literal(Literal::Number(float));
        let res = interpreter.interpret(expression);
        assert_eq!(res, float.to_string());
    }

    #[test]
    fn test_interpret_literal_nil() {
        let mut interpreter = Interpreter::new();
        let expression = Expr::Literal(Literal::Nil);
        let res = interpreter.interpret(expression);
        assert_eq!(res, "nil");
    }
}
