use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}
#[allow(unused)]
#[derive(Debug,Clone)]
pub struct Token {
    tok_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}
impl Token {
    pub fn new(tok_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Self {
            tok_type,
            lexeme,
            literal,
            line,
        }
    }
    pub fn to_string(&self) -> String {
        return format!("{:?} {} {:?}", self.tok_type, self.lexeme, self.literal);
    }
}
