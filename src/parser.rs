use crate::ast::{Expr, Stmt};
use crate::error::RoxError;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType::{
    self, And, Bang, BangEqual, Comma, Else, Eof, Equal, EqualEqual, False, For, Fun, Greater,
    GreaterEqual, Identifier, If, LeftBrace, LeftParen, Less, LessEqual, Minus, Nil, Number, Or,
    Plus, Print, RightBrace, RightParen, Semicolon, Slash, Star, String_, True, Var, While,
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
            statements.push(self.declaration().unwrap());
        }

        statements
    }

    fn statement(&mut self) -> Result<Stmt, RoxError> {
        if self.match_types([For].to_vec()) {
            return self.for_statement();
        }

        if self.match_types([If].to_vec()) {
            return self.if_statement();
        }

        if self.match_types([Print].to_vec()) {
            return self.print_statement();
        }

        if self.match_types([While].to_vec()) {
            return self.while_statement();
        }

        if self.match_types([LeftBrace].to_vec()) {
            return Ok(Stmt::Block(self.block()?));
        }

        return self.expression_statement();
    }

    fn function(&mut self, kind: String) -> Result<Stmt, RoxError> {
        let name = self.consume(
            Identifier,
            "Expect".to_owned() + &kind + &"name.".to_string(),
        )?;

        self.consume(
            LeftParen,
            "Expect ( after".to_owned() + &kind + &"name.".to_string(),
        );

        let mut parameters = Vec::new();

        if !self.check(RightParen) {
            parameters.push(self.consume(Identifier, "Expect parameter name.".to_string())?);
            loop {
                if parameters.len() >= 255 {
                    return Err(RoxError::MaxParameterLimitError);
                } else if !self.match_types([Comma].to_vec()) {
                    break;
                } else {
                    parameters
                        .push(self.consume(Identifier, "Expect parameter name.".to_string())?);
                }
            }
        }

        self.consume(RightParen, "Expect ')' after parameters".to_string());
        self.consume(
            LeftBrace,
            "Expect '{' before".to_owned() + &kind + &"name.".to_string(),
        );

        let body = self.block()?;
        Ok(Stmt::Function(name, parameters, body))
    }

    fn declaration(&mut self) -> Result<Stmt, RoxError> {
        if self.match_types([Fun].to_vec()) {
            return self.function("function".to_string());
        }

        if self.match_types([Var].to_vec()) {
            return self.var_declaration();
        }

        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, RoxError> {
        let token_name = self.consume(Identifier, "Expect variable name.".to_string())?;

        let initializer = if self.match_types([Equal].to_vec()) {
            self.expression().ok()
        } else {
            None
        };
        self.consume(Semicolon, "Expect ';' after value.".to_string())?;

        Ok(Stmt::Var(token_name, initializer))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, RoxError> {
        let mut statements = Vec::new();

        while !self.check(RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(RightBrace, "Expect '}' after block.".to_string())?;
        Ok(statements)
    }

    fn for_statement(&mut self) -> Result<Stmt, RoxError> {
        self.consume(LeftParen, "Expect '(' after 'for'.".to_string())?;

        let initializer = if self.match_types([Semicolon].to_vec()) {
            None
        } else if self.match_types([Var].to_vec()) {
            self.var_declaration().ok()
        } else {
            self.expression_statement().ok()
        };

        let condition = if !self.check(Semicolon) {
            self.expression().ok()
        } else {
            None
        };

        self.consume(Semicolon, "Expect ';' after loop condition.".to_string())?;

        let increment = if !self.check(RightParen) {
            self.expression().ok()
        } else {
            None
        };

        self.consume(RightParen, "Expect ')' after for clauses.".to_string())?;
        let mut body = self.statement()?;
        if increment != None {
            body = Stmt::Block(vec![body, Stmt::Expression(increment.unwrap())]);
        }

        let condition = if condition == None {
            Some(Expr::Literal(Literal::Bool(true)))
        } else {
            condition
        };

        body = Stmt::While(condition.unwrap(), Box::new(body));

        if Some(initializer.as_ref().unwrap()) != None {
            body = Stmt::Block(vec![initializer.unwrap(), body]);
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, RoxError> {
        self.consume(LeftParen, "Expect '(' after if.".to_string())?;
        let condition: Expr = self.expression()?;
        self.consume(RightParen, "Expect ')' after if condition.".to_string())?;

        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.match_types([Else].to_vec()) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(condition, then_branch, else_branch))
    }

    fn while_statement(&mut self) -> Result<Stmt, RoxError> {
        self.consume(LeftParen, "Expect '(' after 'while'.".to_string())?;
        let condition = self.expression()?;
        self.consume(RightParen, "Expect ')' after condition.".to_string())?;

        let body = self.statement()?;

        Ok(Stmt::While(condition, Box::new(body)))
    }

    fn print_statement(&mut self) -> Result<Stmt, RoxError> {
        let value: Expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after value.".to_string())?;
        Ok(Stmt::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Stmt, RoxError> {
        let expr: Expr = self.expression()?;
        self.consume(Semicolon, "Expect ';' after expression.".to_string())?;
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, RoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, RoxError> {
        let expr = self.or()?;

        if self.match_types([Equal].to_vec()) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Var(name) = expr {
                Ok(Expr::Assign(name, Box::new(value)))
            } else {
                Err(RoxError::InvalidAssignmentError(equals))
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr, RoxError> {
        let mut expr = self.and()?;

        while self.match_types([Or].to_vec()) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, RoxError> {
        let mut expr = self.equality()?;

        while self.match_types([And].to_vec()) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
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
        } else {
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, RoxError> {
        let mut expr = self.primary();

        loop {
            if self.match_types([LeftParen].to_vec()) {
                expr = self.finish_call(expr.unwrap());
            } else {
                break;
            }
        }

        expr
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, RoxError> {
        let mut args = Vec::new();
        if !self.check(RightParen) {
            args.push(self.expression()?);
            while self.match_types([Comma].to_vec()) {
                args.push(self.expression()?);
            }
        }
        let paren = self.consume(RightParen, "Expect ')' after arguments.".to_string())?;

        Ok(Expr::Call(Box::new(callee), paren, args))
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

        if self.match_types([Identifier].to_vec()) {
            return Ok(Expr::Var(self.previous()));
        }

        if self.match_types([LeftParen].to_vec()) {
            let expr = self.expression()?;
            self.consume(RightParen, "Expect ')' after expression.".to_string())?;
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

    #[test]
    fn test_parse_if_statement() -> Result<(), RoxError> {
        // if (true) a = 1; else a = 2;
        let tokens = vec![
            Token {
                token_type: If,
                lexeme: "if".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: LeftParen,
                lexeme: "(".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: True,
                lexeme: "true".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: RightParen,
                lexeme: ")".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: Identifier,
                lexeme: "a".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: Equal,
                lexeme: "=".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: Number,
                lexeme: "1".to_string(),
                literal: Some(Literal::Number(1.0)),
                line: 1,
            },
            Token {
                token_type: Semicolon,
                lexeme: ";".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: Else,
                lexeme: "else".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: Identifier,
                lexeme: "a".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: Equal,
                lexeme: "=".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: Number,
                lexeme: "2".to_string(),
                literal: Some(Literal::Number(2.0)),
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
                line: 3,
            },
        ];

        let mut parser = Parser::new(tokens.clone());
        parser.parse();
        Ok(())
    }
}
