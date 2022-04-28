use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenType::{
    And, Bang, BangEqual, Class, Comma, Dot, Else, Eof, Equal, EqualEqual, False, For, Fun,
    Greater, GreaterEqual, Identifier, If, LeftBrace, LeftParen, Less, LessEqual, Minus, Nil,
    Number, Or, Plus, Print, Return, RightBrace, RightParen, Semicolon, Slash, Star, String_,
    Super, This, True, Var, While,
};

use crate::error::RoxError;

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("and".to_owned(), And);
        m.insert("class".to_owned(), Class);
        m.insert("else".to_owned(), Else);
        m.insert("false".to_owned(), False);
        m.insert("for".to_owned(), For);
        m.insert("fun".to_owned(), Fun);
        m.insert("if".to_owned(), If);
        m.insert("nil".to_owned(), Nil);
        m.insert("or".to_owned(), Or);
        m.insert("print".to_owned(), Print);
        m.insert("return".to_owned(), Return);
        m.insert("super".to_owned(), Super);
        m.insert("this".to_owned(), This);
        m.insert("true".to_owned(), True);
        m.insert("var".to_owned(), Var);
        m.insert("while".to_owned(), While);
        m
    };
}

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(Eof, "", None, self.line));

        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let token = Token::new(
            token_type,
            &self.source[self.start..self.current],
            literal,
            self.line,
        );
        self.tokens.push(token);
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let token_type = if self.match_char('=') {
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(token_type)
            }
            '=' => {
                let token_type = if self.match_char('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(token_type)
            }
            '<' => {
                let token_type = if self.match_char('=') {
                    LessEqual
                } else {
                    Less
                };
                self.add_token(token_type)
            }
            '>' => {
                let token_type = if self.match_char('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(token_type)
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                };
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            'o' => {
                if self.match_char('r') {
                    self.add_token(Or);
                }
            }
            _ => {
                if self.is_digit(c) {
                    self.number()
                } else if self.is_alphanumeric(c) {
                    self.identifier()
                } else {
                    RoxError::UnexpectedCharacterError(self.line.to_string());
                }
            }
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() == expected {
            self.current += 1;
            return true;
        } else {
            return false;
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            RoxError::UnexpectedCharacterError(self.line.to_string());
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_owned();
        let literal = Literal::String_(value);
        self.add_token_with_literal(String_, Some(literal));
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let fractional_part =
            Literal::Number(f64::from_str(&self.source[(self.start)..(self.current)]).unwrap());
        self.add_token_with_literal(Number, Some(fractional_part))
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[(self.start)..(self.current)];
        let token_type = KEYWORDS
            .get(text)
            .map_or_else(|| Identifier, std::clone::Clone::clone);
        self.add_token(token_type)
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_tokens() {
        let mut scanner = Scanner::new("print 'Hello, world!'".to_string());
        scanner.scan_tokens();

        let expected_tokens = vec![
            Token::new(Print, "print", None, 1),
            Token::new(Identifier, "Hello", None, 1),
            Token::new(Comma, ",", None, 1),
            Token::new(Identifier, "world", None, 1),
            Token::new(Bang, "!", None, 1),
            Token::new(Eof, "", None, 1),
        ];
        assert!(scanner.tokens.len() == 6);

        for i in 0..scanner.tokens.len() {
            assert!(scanner.tokens[i] == expected_tokens[i]);
        }
    }

    #[test]
    fn test_scan_token() {
        let mut scanner = Scanner::new("sc".to_string());
        scanner.scan_token();

        let expected_token_1 = Token::new(Identifier, "sc", None, 1);

        assert!(scanner.tokens.len() == 1);
        assert!(scanner.tokens[0] == expected_token_1);
    }

    #[test]
    fn test_add_token() {
        let mut scanner = Scanner::new("sc".to_string());
        let expected_token_1 = Token::new(LeftParen, "", None, 1);
        scanner.add_token(LeftParen);

        assert!(scanner.tokens[0] == expected_token_1);
    }

    #[test]
    fn test_peek() {
        let scanner = Scanner::new("p".to_string());

        assert!(scanner.peek() == 'p');
    }

    #[test]
    fn test_number() {
        let mut scanner = Scanner::new("314 == 'pi'".to_string());
        scanner.scan_tokens();
        let expected_tokens = vec![
            Token::new(Number, "314", Some(Literal::Number(1.0)), 1),
            Token::new(EqualEqual, "==", None, 1),
            Token::new(Identifier, "pi", None, 1),
            Token::new(Eof, "", None, 1),
        ];
        for i in 0..scanner.tokens.len() {
            assert!(scanner.tokens[i] == expected_tokens[i]);
        }
    }

    #[test]
    fn test_is_at_end() {
        let scanner = Scanner::new("".to_string());

        assert!(scanner.is_at_end());
    }
}
