use std::fmt;

use crate::token::Token;
use crate::token::TokenType::Eof;

#[derive(Debug)]
pub enum RoxError {
    UnexpectedCharacterError(String),
    ParseError(Token, String),
    RuntimeError(Token, String),
    UnexpectedError,
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
            RoxError::RuntimeError(token, message) => {
                write!(f, "{}", message)
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
