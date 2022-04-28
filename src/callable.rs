use crate::ast::Value;
use crate::Interpreter;

pub trait RoxCallable {
    fn call(interpreter: Interpreter, arguments: Vec<Value>);
}
