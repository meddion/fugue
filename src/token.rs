use crate::lexer::*;
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
    Number(f64),
    Str(String),
}

impl Token {
    pub fn match_lexem(lexem: &str) -> Option<&Token> {
        TOKEN_MAP.get(lexem)
    }
}

pub fn next_is_equal_to(tokens_iter: &mut LexerIter, expected: &Token) -> bool {
    match tokens_iter.peek() {
        Some(&token) if token == expected => true,
        _ => false,
    }
}

pub fn next_is_of_type(tokens_iter: &mut LexerIter, expected: &Token) -> bool {
    use std::mem::discriminant;
    match tokens_iter.peek() {
        Some(&token) => discriminant(token) == discriminant(expected),
        _ => false,
    }
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

// pub struct TokenExt {
//     token: Token,
//     line: u64,
//     col: u64,
// }
