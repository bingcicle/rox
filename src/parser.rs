use crate::ast::{Expr, UnaryOperator};
use crate::error::RoxError;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType::{
    self, Bang, BangEqual, Eof, EqualEqual, False, Greater, GreaterEqual, Identifier, LeftParen,
    Less, LessEqual, Minus, Nil, Number, Plus, RightParen, Slash, Star, String_, True,
};
use std::result::Result;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, RoxError> {
        self.expression()
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
        for token_type in token_types {
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

    #[test]
    fn test_consume() {
        let tokens = vec![Token::new(LeftParen, "(", None, 1)];

        let mut parser = Parser::new(tokens.clone());
        let res = parser.consume(tokens[0].clone().token_type, "message".to_string());

        assert!(res.is_ok())
    }
}
