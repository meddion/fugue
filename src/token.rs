use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Skip,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LineDelim,
    // Semicolon,
    Comma,
    Dot,

    Minus,
    Plus,
    Slash,
    Star,

    Neg,
    NegEqual,
    Equal,
    DoubleEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Keywords
    Let,
    If,
    Else,
    Loop,
    Return,
    EOF,

    // Literals
    Ident(String),
    Int(i64),
    Float(f64),
    Str(String),
}

lazy_static! {
    static ref TOKEN_MAP: HashMap<&'static str, Token> = vec![
        ("let", Token::Let),
        ("if", Token::If),
        ("else", Token::Else),
        ("loop", Token::Loop),
        ("return", Token::Return),
    ]
    .into_iter()
    .collect();
}

pub struct TokenExt {
    token: Token,
    line: u64,
    col: u64,
}

#[derive(Eq, PartialEq)]
pub enum Cursor {
    Move,
    Keep,
}

impl Token {
    pub fn match_lexem(lexem: &str) -> Option<&Token> {
        TOKEN_MAP.get(lexem)
    }
}
