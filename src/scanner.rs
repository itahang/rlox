use std::collections::HashMap;

use super::errors::*;
use super::token::*;
use super::token_type::*;

/// A scanner that converts source code into a sequence of tokens.
pub struct Scanner {
    /// Source code represented as UTF-8 bytes.
    source: Vec<u8>,

    /// Tokens produced by the scanner.
    tokens: Vec<Token>,

    /// Vectors of errors

    /// Byte index marking the start of the current lexeme.
    start: usize,

    /// Byte index of the current position in the source.
    current: usize,

    /// Current line number, used for error reporting.
    line: usize,

    key_words: HashMap<String, TokenType>,
}

impl Scanner {
    /// Creates a new `Scanner` from the given source code.
    pub fn new(source: String) -> Self {
        let mut kw: HashMap<String, TokenType> = HashMap::new();
        kw.insert("and".to_string(), TokenType::AND);
        kw.insert("class".to_string(), TokenType::CLASS);
        kw.insert("else".to_string(), TokenType::ELSE);
        kw.insert("false".to_string(), TokenType::FALSE);
        kw.insert("for".to_string(), TokenType::FOR);
        kw.insert("fun".to_string(), TokenType::FUN);
        kw.insert("if".to_string(), TokenType::IF);
        kw.insert("nil".to_string(), TokenType::NIL);
        kw.insert("or".to_string(), TokenType::OR);
        kw.insert("print".to_string(), TokenType::PRINT);
        kw.insert("return".to_string(), TokenType::RETURN);
        kw.insert("super".to_string(), TokenType::SUPER);
        kw.insert("this".to_string(), TokenType::THIS);
        kw.insert("true".to_string(), TokenType::TRUE);
        kw.insert("var".to_string(), TokenType::VAR);
        kw.insert("while".to_string(), TokenType::WHILE);

        Self {
            source: source.into_bytes(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            key_words: kw,
        }
    }

    /// Returns `true` if the scanner has reached the end of the source.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return 0;
        }
        return self.source[self.current];
    }
    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.source.len() {
            return 0;
        }
        return self.source[self.current + 1];
    }
    fn advance(&mut self) -> u8 {
        let ch = self.source[self.current];
        self.current += 1;
        return ch;
    }

    fn add_token(&mut self, tok_type: TokenType, lexeme: String, literal: Literal, line: usize) {
        self.tokens
            .push(Token::new(tok_type, lexeme, literal, line));
    }

    fn match_next(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;

        return true;
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(report(self.line, "", "Unterminated String.").into());
        }
        self.advance();

        let value =
            String::from_utf8(self.source[self.start + 1..self.current - 1].to_vec()).unwrap();
        self.add_token(
            TokenType::STRING,
            value.clone(),
            Literal::String(value),
            self.line,
        );
        return Ok(());
    }

    fn is_digit(&self, ch: u8) -> bool {
        return ch >= b'0' && ch <= b'9';
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == b'.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let num = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();

        self.add_token(
            TokenType::NUMBER,
            num.clone(),
            Literal::Number(num.parse().unwrap()),
            self.line,
        );
    }

    fn is_alphabet(&self, c: u8) -> bool {
        return (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_';
    }

    fn is_alphanumerical(&self, c: u8) -> bool {
        return self.is_alphabet(c) || self.is_digit(c);
    }

    fn identifier(&mut self) {
        while self.is_alphanumerical(self.peek()) {
            self.advance();
        }
        let text = String::from_utf8(self.source[self.start..self.current].to_vec()).unwrap();
        let tok_type = match self.key_words.get(&text).cloned() {
            Some(tt) => tt,
            None => TokenType::IDENTIFIER,
        };

        self.add_token(tok_type, text.clone(), Literal::String(text), self.line);
    }

    /// Each run creats a single `Token` and push to `self.tokens`
    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();

        match c {
            b'(' => self.add_token(
                TokenType::LEFT_PAREN,
                "".to_string(),
                Literal::Nil,
                self.line,
            ),
            b')' => self.add_token(
                TokenType::RIGHT_PAREN,
                "".to_string(),
                Literal::Nil,
                self.line,
            ),
            b'{' => self.add_token(
                TokenType::LEFT_BRACE,
                "".to_string(),
                Literal::Nil,
                self.line,
            ),
            b'}' => self.add_token(
                TokenType::RIGHT_BRACE,
                "".to_string(),
                Literal::Nil,
                self.line,
            ),
            b',' => self.add_token(TokenType::COMMA, "".to_string(), Literal::Nil, self.line),
            b'.' => self.add_token(TokenType::DOT, "".to_string(), Literal::Nil, self.line),
            b'-' => self.add_token(TokenType::MINUS, "".to_string(), Literal::Nil, self.line),
            b'+' => self.add_token(TokenType::PLUS, "".to_string(), Literal::Nil, self.line),
            b';' => self.add_token(
                TokenType::SEMICOLON,
                "".to_string(),
                Literal::Nil,
                self.line,
            ),
            b'*' => self.add_token(TokenType::STAR, "".to_string(), Literal::Nil, self.line),

            b'!' => {
                let token_type = if self.match_next(b'=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(token_type, "".to_string(), Literal::Nil, self.line);
            }

            b'=' => {
                let token_type = if self.match_next(b'=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(token_type, "".to_string(), Literal::Nil, self.line)
            }

            b'<' => {
                let token_type = if self.match_next(b'=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(token_type, "".to_string(), Literal::Nil, self.line)
            }

            b'>' => {
                let token_type = if self.match_next(b'=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };

                self.add_token(token_type, "".to_string(), Literal::Nil, self.line)
            }

            b'/' => {
                if self.match_next(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, "".to_string(), Literal::Nil, self.line);
                }
            }
            b' ' | b'\r' | b'\t' => {}

            b'\n' => self.line += 1,

            b'"' => {
                self.string()?;
            }

            b'o' => {
                if self.match_next(b'r') {
                    self.add_token(TokenType::OR, "".to_string(), Literal::Nil, self.line);
                }
            }

            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alphabet(c) {
                    self.identifier();
                } else {
                    return Err(report(self.line, "", "Unexpected character!").into());
                }
            }
        }
        Ok(())
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            Literal::Nil,
            self.line,
        ));

        Ok(self.tokens.clone())
    }
}
