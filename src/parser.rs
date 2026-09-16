
use crate::errors::{LoxError, ParserError};
use crate::expression::Expr;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    fn parser_error(&self, token: &Token, message: &str) -> ParserError {
        if token.get_type() == TokenType::EOF {
            ParserError::new(token.get_line(), "at end".to_string(), message.to_string())
        } else {
            ParserError::new(
                token.get_line(),
                format!("at '{}'", token.get_lexeme_string()),
                message.to_string(),
            )
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }
    fn is_at_end(&self) -> bool {
        self.peek().get_type() == TokenType::EOF
    }

    fn check(&self, ctype: &TokenType) -> bool {
        self.peek().get_type() == *ctype
    }

    fn match_next(&mut self, tokens: &Vec<TokenType>) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        return false;
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    pub fn expression(&mut self) -> Result<Expr, LoxError> {
        return self.equality();
    }
    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparision()?;
        let to_match: Vec<TokenType> = vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL];
        while self.match_next(&to_match) {
            let operator = self.previous();
            let right = self.comparision()?;
            let tmp = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
            expr = tmp;
        }

        return Ok(expr);
    }

    fn comparision(&mut self) -> Result<Expr, LoxError> {
        let mut exp = self.term()?;
        let to_match: Vec<TokenType> = vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ];
        while self.match_next(&to_match) {
            let operator = self.previous();
            let right = self.term()?;
            let tmp = Expr::Binary {
                left: Box::new(exp),
                operator,
                right: Box::new(right),
            };
            exp = tmp;
        }

        return Ok(exp);
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut exp = self.unary()?;
        let to_match = vec![TokenType::MINUS, TokenType::PLUS];
        while self.match_next(&to_match) {
            let operator = self.previous();
            let right = self.unary()?;
            let tmp = Expr::Binary {
                left: Box::new(exp),
                operator,
                right: Box::new(right),
            };

            exp = tmp;
        }

        return Ok(exp);
    }
    fn unary(&mut self) -> Result<Expr, LoxError> {
        match self.peek().get_type() {
            TokenType::BANG | TokenType::MINUS => {
                let operator = self.advance();
                return Ok(Expr::Unary {
                    operator,
                    right: Box::new(self.unary()?),
                });
            }

            _ => return Ok(self.primary()?),
        }
    }

    fn consume(&mut self, tt: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(&tt) {
            Ok(self.advance())
        } else {
            Err(self.parser_error(&self.peek(), message).into())
        }
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        match self.peek().get_type() {
            TokenType::FALSE
            | TokenType::TRUE
            | TokenType::NIL
            | TokenType::NUMBER
            | TokenType::STRING => {
                let token = self.advance();
                Ok(Expr::Literal {
                    value: token.get_literal(),
                })
            }

            TokenType::LEFT_PAREN => {
                self.advance();

                let exp = self.expression()?;

                self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.")?;

                Ok(Expr::Grouping {
                    expression: Box::new(exp),
                })
            }

            _ => Err(self.parser_error(&self.peek(), "incorrect Parse").into()),
        }
    }
}
