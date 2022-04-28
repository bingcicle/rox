use std::fmt;

use crate::token::Token;
use crate::token::TokenType::Eof;

#[derive(Debug)]
pub enum RoxError {
    UnexpectedCharacterError(String),
    ParseError(Token, String),
    RuntimeError(String),
    UndefinedVariableError(Token),
    InvalidAssignmentError(Token),
    UnexpectedError,
    MaxParameterLimitError,
}

impl fmt::Display for RoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RoxError::UnexpectedCharacterError(line_str) => {
                write!(f, "Unexpected character at {}", line_str)
            }
            RoxError::ParseError(token, message) => {
                if token.token_type == Eof {
                    return write!(f, "{} at end {}", token.line, message);
                } else {
                    return write!(f, "{} at '{}' {}", token.line, token.lexeme, message);
                }
            }
            RoxError::UndefinedVariableError(token) => {
                write!(f, "Undefined variable '{}'.", token.lexeme)
            }
            RoxError::RuntimeError(message) => {
                write!(f, "{}", message)
            }
            RoxError::InvalidAssignmentError(token) => {
                write!(f, "Invalid assignment target {}.", token.lexeme)
            }
            RoxError::MaxParameterLimitError => {
                write!(f, "Can't have more than 255 parameters.")
            }
            RoxError::UnexpectedError => {
                write!(f, "Unexpected error while parsing")
            }
        }
    }
}

pub struct ErrorHandler {}

impl ErrorHandler {
    fn report(line: i64, location: String, message: String) {
        println!("[line {}] Error {} ': {}", line, location, message)
    }
}
