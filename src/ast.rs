use crate::function::RoxFunction;
use crate::token::{Literal, Token};
use std::fmt;

pub enum UnaryOperator {
    Bang,
    Minus,
}

#[derive(Clone)]
pub enum Value {
    String_(String),
    Bool(bool),
    Number(f64),
    Callable(RoxFunction),
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Value {
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (_, Value::Nil) => false,
            (Value::Nil, _) => false,
            (Value::Bool(left), Value::Bool(right)) => left == right,
            (Value::Number(left), Value::Number(right)) => left == right,
            (Value::String_(left), Value::String_(right)) => left.eq(right),
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Var(Token),
    Assign(Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Print(Expr),
    Expression(Expr),
    Block(Vec<Stmt>),
    Var(Token, Option<Expr>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    Function(Token, Vec<Token>, Vec<Stmt>),
}

pub trait StmtVisitor<Value> {
    fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr) => self.visit_expr_stmt(expr),
            Stmt::Print(expr) => self.visit_print_stmt(expr),
            Stmt::Var(token, expr) => self.visit_var_stmt(token, expr),
            Stmt::Block(stmts) => self.visit_block_stmt(stmts),
            Stmt::If(expr, then_stmt, else_stmt) => self.visit_if_stmt(expr, then_stmt, else_stmt),
            Stmt::While(expr, body_stmt) => self.visit_while_stmt(expr, body_stmt),
            Stmt::Function(name, params, body) => self.visit_function_stmt(name, params, body),
        }
    }

    fn visit_expr_stmt(&mut self, stmt_expr: Expr);
    fn visit_print_stmt(&mut self, stmt_expr: Expr);
    fn visit_var_stmt(&mut self, token: Token, stmt_expr: Option<Expr>);
    fn visit_block_stmt(&mut self, statements: Vec<Stmt>);
    fn visit_if_stmt(&mut self, expr: Expr, then_stmt: Box<Stmt>, else_stmt: Option<Box<Stmt>>);
    fn visit_while_stmt(&mut self, expr: Expr, body_stmt: Box<Stmt>);
    fn visit_function_stmt(&mut self, name: Token, params: Vec<Token>, body: Vec<Stmt>);
}

pub trait ExprVisitor<Value> {
    fn evaluate(&mut self, expr: Expr) -> Value {
        match expr {
            Expr::Literal(l) => self.visit_literal_expr(l),
            Expr::Unary(op, r) => self.visit_unary_expr(op, r),
            Expr::Binary(l, op, r) => self.visit_binary_expr(l, op, r),
            Expr::Grouping(g) => self.visit_grouping_expr(g),
            Expr::Var(t) => self.visit_var_expr(t),
            Expr::Assign(t, expr) => self.visit_assignment_expr(t, expr),
            Expr::Logical(l, op, r) => self.visit_logical_expr(l, op, r),
            Expr::Call(c, p, a) => self.visit_call_expr(c, p, a),
        }
    }

    fn visit_literal_expr(&mut self, literal: Literal) -> Value;
    fn visit_grouping_expr(&mut self, grouping_expr: Box<Expr>) -> Value;
    fn visit_unary_expr(&mut self, operator: Token, right: Box<Expr>) -> Value;
    fn visit_binary_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Value;
    fn visit_var_expr(&mut self, name: Token) -> Value;
    fn visit_assignment_expr(&mut self, name: Token, expr: Box<Expr>) -> Value;
    fn visit_logical_expr(&mut self, left: Box<Expr>, operator: Token, right: Box<Expr>) -> Value;
    fn visit_call_expr(&mut self, callee: Box<Expr>, paren: Token, args: Vec<Expr>) -> Value;
    fn is_truthy(&mut self, value: Value) -> bool;
    fn is_equal(&mut self, a: Value, b: Value) -> bool;
}
