use crate::ast::{Expr, Stmt, UnaryOperator};
use crate::error::RoxError;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType::{
    self, Bang, BangEqual, Eof, EqualEqual, False, Greater, GreaterEqual, Identifier, LeftParen,
    Less, LessEqual, Minus, Nil, Number, Plus, Print, RightParen, Semicolon, Slash, Star, String_,
    True,
};
use std::result::Result;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.statement().unwrap());
        }

        println!("statements: {:?}", statements);

        statements
    }

    fn statement(&mut self) -> Result<Stmt, RoxError> {
        println!("{:?}", self.tokens);
        if self.match_types([Print].to_vec()) {
            return self.print_statement();
        }

        return self.expression_statement();
    }

    fn print_statement(&mut self) -> Result<Stmt, RoxError> {
        let value: Expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.".to_string());
        Ok(Stmt::Print(value))
    }
    fn expression_statement(&mut self) -> Result<Stmt, RoxError> {
        let mut expr: Expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after expression.".to_string());
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, RoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, RoxError> {
        let mut expr: Expr = self.comparison()?;
        while self.match_types([BangEqual, EqualEqual].to_vec()) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right))
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, RoxError> {
        let mut expr: Expr = self.term()?;

        while self.match_types([Greater, GreaterEqual, Less, LessEqual].to_vec()) {
            let operator: Token = self.previous();
            let right: Expr = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, RoxError> {
        let mut expr: Expr = self.factor()?;

        while self.match_types([Minus, Plus].to_vec()) {
            let operator: Token = self.previous();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, RoxError> {
        let mut expr: Expr = self.unary()?;

        while self.match_types([Slash, Star].to_vec()) {
            let operator: Token = self.previous();
            let right: Expr = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, RoxError> {
        if self.match_types([Bang, Minus].to_vec()) {
            let operator: Token = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, RoxError> {
        if self.match_types([False].to_vec()) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.match_types([True].to_vec()) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }

        if self.match_types([Nil].to_vec()) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_types([Number, String_].to_vec()) {
            return Ok(Expr::Literal(match self.previous().literal {
                Some(l) => l,
                None => Literal::Nil,
            }));
        }

        if self.match_types([LeftParen].to_vec()) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression.".to_string());
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(RoxError::UnexpectedError)
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, RoxError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(RoxError::ParseError(self.peek(), message))
        }
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == Eof;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous().clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn match_types(&mut self, token_types: Vec<TokenType>) -> bool {
        println!("{:?}", token_types);
        for token_type in token_types {
            println!("{:?} {:?}", &token_type, self.peek().token_type);
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == token_type;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast;

    #[test]
    fn test_consume() {
        let tokens = vec![Token::new(LeftParen, "(", None, 1)];

        let mut parser = Parser::new(tokens.clone());
        let res = parser.consume(tokens[0].clone().token_type, "message".to_string());

        assert!(res.is_ok())
    }

    #[test]
    fn test_parse_expression_statement() {
        let tokens = vec![
            Token {
                token_type: Print,
                lexeme: "print".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: String_,
                lexeme: "one".to_string(),
                literal: Some(Literal::String_("one".to_string())),
                line: 1,
            },
            Token {
                token_type: Semicolon,
                lexeme: ";".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: Eof,
                lexeme: "".to_string(),
                literal: None,
                line: 2,
            },
        ];

        let mut parser = Parser::new(tokens.clone());
        let statements = parser.parse();

        let expected_statement =
            Stmt::Print(ast::Expr::Literal(Literal::String_("one".to_string())));

        assert!(statements[0] == expected_statement);
    }
}
