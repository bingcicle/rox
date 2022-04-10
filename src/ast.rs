use crate::token::{Literal, Token};

pub enum UnaryOperator {
    Bang,
    Minus,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    String_(String),
    Bool(bool),
    Number(f64),
    Nil,
}

impl From<Literal> for Value {
    fn from(l: Literal) -> Self {
        match l {
            Literal::String_(s) => Self::String_(s),
            Literal::Bool(b) => Self::Bool(b),
            Literal::Number(n) => Self::Number(n),
            Literal::Nil => Self::Nil,
        }
    }
}

pub enum Expr {
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
}

pub trait ExprVisitor<Value> {
    fn evaluate(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Literal(l) => self.visit_literal_expr(l),
            Expr::Unary(op, r) => self.visit_unary_expr(op, r),
            Expr::Binary(l, op, r) => self.visit_binary_expr(l, op, r),
            Expr::Grouping(g) => self.visit_grouping_expr(g),
        }
    }

    fn visit_literal_expr(&mut self, literal: Literal) -> Value;
    fn visit_grouping_expr(&mut self, grouping_expr: Box<Expr>) -> Value;
    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> Value;
    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Value;
    fn is_truthy(&mut self, value: Value) -> bool;
    fn is_equal(&mut self, a: Value, b: Value) -> bool;
}
