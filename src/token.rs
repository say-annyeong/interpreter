use std::collections::HashMap;
use lazy_static::lazy_static;

pub type TokenType = &'static str;

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";
pub const IDENT: TokenType = "IDENT";
pub const INT: TokenType = "INT";
pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";
pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";
pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";
pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";

lazy_static!(
    static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([("fn", FUNCTION), ("let", LET)]);
);

#[derive(Debug)]
pub struct Token {
    pub token_type: String,
    pub literal: String
}

impl Token {
    pub fn new(token_type: String, literal: String) -> Self {
        Token { token_type, literal }
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    if let Some(&tok) = KEYWORDS.get(ident) {
        tok
    } else {
        IDENT
    }
}
