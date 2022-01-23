use crate::tokens::{next_is_equal_to, next_is_of_type, Token};

use std::io::{BufRead, BufReader};
use std::io::{ErrorKind, Read, Result};
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer {
    pub tokens: Vec<Token>,
    _line: u64,
    _column: u64,
}

macro_rules! _next {
    ($iter:ident, $token:expr) => {{
        $iter.next();
        Ok($token)
    }};
}

pub type TokenStream<'a> = <&'a Lexer as IntoIterator>::IntoIter;

impl<'a> IntoIterator for &'a Lexer {
    type Item = &'a Token;
    type IntoIter = Peekable<std::slice::Iter<'a, Token>>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.as_slice().iter().peekable()
    }
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            _line: 0,
            _column: 0,
        }
    }

    pub fn clear(&mut self) {
        self.tokens = Vec::new()
    }

    pub fn scan<R: Read>(&mut self, reader: R) -> std::io::Result<()> {
        let input = BufReader::new(reader);
        // TODO: add line number handling
        for (_, line) in input.lines().enumerate() {
            if let Ok(mut line) = line {
                self.add_tokens_from_string(&mut line)?;
            }
        }

        self.tokens.push(Token::EOF);
        Ok(())
    }

    fn add_tokens_from_string(&mut self, str: &mut String) -> Result<()> {
        let mut str_iter = str.chars().peekable();
        while let Some(ch) = str_iter.peek() {
            let token = match ch {
                '(' => _next!(str_iter, Token::LeftParen),
                ')' => _next!(str_iter, Token::RightParen),
                '[' => _next!(str_iter, Token::LeftBrace),
                ']' => _next!(str_iter, Token::RightBrace),
                '*' => _next!(str_iter, Token::Star),
                '+' => _next!(str_iter, Token::Plus),
                '-' => _next!(str_iter, Token::Minus),
                '/' => _next!(str_iter, Token::Slash),
                '.' => _next!(str_iter, Token::Dot),
                ',' => _next!(str_iter, Token::Comma),
                '=' => Lexer::scan_equal(&mut str_iter),
                '"' => Lexer::scan_string(&mut str_iter),
                ch if Lexer::is_line_delim(*ch) => _next!(str_iter, Token::LineDelim),
                ch if ch.is_whitespace() => _next!(str_iter, Token::Skip),
                ch if ch.is_numeric() => Lexer::scan_number(&mut str_iter),
                ch if ch.is_alphanumeric() => Lexer::scan_word(&mut str_iter),

                _ => return Err(ErrorKind::InvalidData.into()),
            };

            match token {
                Err(err) => return Err(err),
                Ok(token) if token != Token::Skip => self.tokens.push(token),
                _ => (),
            }
        }
        Ok(())
    }

    fn is_line_delim(c: char) -> bool {
        c == '\n' || c == ';'
    }

    fn scan_equal(str_iter: &mut Peekable<Chars<'_>>) -> Result<Token> {
        str_iter.next();
        Ok(match str_iter.peek() {
            Some('=') => Token::DoubleEqual,
            _ => Token::Equal,
        })
    }

    fn scan_string(str_iter: &mut Peekable<Chars<'_>>) -> Result<Token> {
        str_iter.next();
        let mut new_string = String::new();
        for c in str_iter {
            match c {
                c if Lexer::is_line_delim(c) => return Err(ErrorKind::UnexpectedEof.into()),
                '"' => return Ok(Token::Str(new_string)),
                _ => new_string.push(c),
            }
        }

        Err(ErrorKind::UnexpectedEof.into())
    }

    fn scan_word(str_iter: &mut Peekable<Chars<'_>>) -> Result<Token> {
        let mut ident_name: String = String::new();
        loop {
            ident_name.push(str_iter.next().unwrap());
            match str_iter.peek() {
                Some(ch) if !ch.is_alphanumeric() => break,
                None => break,
                _ => (),
            }
        }

        Ok(match Token::match_lexem(ident_name.as_str()) {
            Some(token) => token.clone(),
            _ => Token::Ident(ident_name),
        })
    }

    fn scan_number(str_iter: &mut Peekable<Chars<'_>>) -> Result<Token> {
        let mut num_string = String::new();

        loop {
            num_string.push(str_iter.next().unwrap());

            match str_iter.peek() {
                Some(ch) if !ch.is_numeric() => break,
                Some('.') => continue,
                None => break,
                _ => (),
            }
        }

        Ok(Token::Number(num_string.parse::<f64>().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Result;

    fn vectors_equal(a: &Vec<Token>, b: &Vec<Token>) -> bool {
        if a.len() != b.len() {
            false
        } else {
            a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count() == a.len()
        }
    }

    #[test]
    fn test_file() -> Result<()> {
        let file = File::open("./misc/lexer_test1.fg")?;

        let mut lexer = Lexer::new();
        lexer.scan(file)?;

        let expected = vec![
            Token::Let,
            Token::Ident("b".to_string()),
            Token::Equal,
            Token::Str("💘💘 Send you my love 😘".to_string()),
            Token::LineDelim,
            Token::EOF,
        ];

        assert!(vectors_equal(&expected, &lexer.tokens));

        Ok(())
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Kind(UnexpectedEof)")]
    fn test_incomplete_string() {
        let mut lexer = Lexer::new();
        let test_string: &[u8] = "let var1 = \"human".as_bytes();
        lexer.scan(test_string).unwrap();
    }

    #[test]
    fn test_utf8_string() {
        let mut lexer = Lexer::new();
        let test_string: &[u8] = "\"💘💘 YOU LOOK SO CHARMING TODAY 💘💘\"".as_bytes();
        lexer.scan(test_string).unwrap();
        let expected = vec![
            Token::Str("💘💘 YOU LOOK SO CHARMING TODAY 💘💘".to_string()),
            Token::EOF,
        ];

        assert!(vectors_equal(&expected, &lexer.tokens));
    }

    #[test]
    fn test_all_tokens() {
        let mut lexer = Lexer::new();
        let keywords: &[u8] = "let if else loop return ;".as_bytes();
        lexer.scan(keywords).unwrap();
        let expected = vec![
            Token::Let,
            Token::If,
            Token::Else,
            Token::Loop,
            Token::Return,
            Token::LineDelim,
            Token::EOF,
        ];
        assert!(vectors_equal(&expected, &lexer.tokens));

        let mut token_iter = lexer.into_iter();
        for token in &expected {
            assert!(next_is_equal_to(&mut token_iter, token.clone()));
            token_iter.next().unwrap();
        }

        lexer.clear();
        assert!(vectors_equal(&vec![], &lexer.tokens));

        let maths: &[u8] = "()[]-+/*; math.sqrt([1.5, 2, 3.0])".as_bytes();
        lexer.scan(maths).unwrap();

        let expected = vec![
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Minus,
            Token::Plus,
            Token::Slash,
            Token::Star,
            Token::LineDelim,
            Token::Ident("math".to_string()),
            Token::Dot,
            Token::Ident("sqrt".to_string()),
            Token::LeftParen,
            Token::LeftBrace,
            Token::Number(1.5),
            Token::Comma,
            Token::Number(2.0),
            Token::Comma,
            Token::Number(3.0),
            Token::RightBrace,
            Token::RightParen,
            Token::EOF,
        ];

        // Is Int(1) == Int(2) ?
        // assert_eq!(Token::Int(1), *token_iter.skip(14).next().unwrap());
        // assert!(next_is_of_type(
        //     // token_iter.skip(11).peekable(),
        //     &Token::Ident("sqrt".to_string()),
        // ));
        assert!(vectors_equal(&expected, &lexer.tokens));
    }

    #[test]
    fn test_value_class_matching() {
        let mut lexer = Lexer::new();

        let expected = vec![
            Token::Ident("math".to_string()),
            Token::Number(24.0),
            Token::Str("Hello world".to_string()),
        ];

        lexer.tokens = expected.clone();
        let mut token_iter = lexer.into_iter();

        for token in &expected {
            assert!(next_is_equal_to(&mut token_iter, token.clone()));
            token_iter.next().unwrap();
        }

        lexer.tokens = expected.clone();
        let mut token_iter = lexer.into_iter();

        let expected_types = vec![
            Token::Ident("Computer Science".to_string()),
            Token::Number(42.0),
            Token::Str("Hello".to_string()),
        ];
        for token in &expected_types {
            assert!(next_is_of_type(&mut token_iter, token.clone()));
            token_iter.next().unwrap();
        }
    }
}
