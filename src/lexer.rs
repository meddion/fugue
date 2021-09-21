use crate::token::*;

use std::io::BufReader;
use std::io::{ErrorKind, Read, Result};
use std::iter::Peekable;
use std::str::Chars;
use std::str::{from_utf8, from_utf8_unchecked};

const BUFF_SIZE: usize = 512;

pub struct Lexer {
    pub tokens: Vec<Token>,
    line: u64,
    column: u64,
    carry_string: String,
}

pub type TokenHandler =
    &'static dyn Fn(&mut String, Option<&mut Peekable<Chars<'_>>>) -> Result<Token>;

macro_rules! _next {
    ($iter:ident, $token:expr) => {{
        $iter.next();
        $token
    }};
}

impl IntoIterator for Lexer {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            line: 0,
            column: 0,
            carry_string: String::new(),
        }
    }

    pub fn parse<R: Read>(&mut self, reader: R) -> std::io::Result<()> {
        let mut input = BufReader::new(reader);

        let mut buf = [0; BUFF_SIZE];
        let mut valid_until = 0;
        let mut bytes_read = 0;

        let mut callback: Option<TokenHandler> = None;

        loop {
            let offset = bytes_read - valid_until;

            for (i, j) in (0..offset).zip(valid_until..) {
                buf[i] = buf[j];
            }

            bytes_read = input.read(&mut buf[offset..])?;
            // FIXME: handle EOF
            if bytes_read == 0 {
                break;
            }
            bytes_read += offset;
            valid_until = bytes_read;

            let mut char_iter = match from_utf8(&buf[..bytes_read]) {
                Ok(s) => s,
                Err(e) => {
                    valid_until = e.valid_up_to();

                    let s = unsafe { from_utf8_unchecked(&buf[..valid_until]) };
                    s
                }
            }
            .chars()
            .peekable();

            // println!("{:?}", self.tokens);

            // TODO: handle callback
            if let Some(cb) = callback.take() {
                match cb(&mut self.carry_string, Some(&mut char_iter)) {
                    Ok(token) => self.add_token(token),
                    _ => {
                        callback = Some(cb);
                        continue;
                    }
                };
            }

            callback = self.get_tokens(&mut char_iter);
        }

        if callback.is_some() {
            // println!("carry_string: {}", self.carry_string);
            // println!("tokens: {:?}", self.tokens);
            let token = callback.unwrap()(&mut self.carry_string, None)?;
            self.add_token(token);
        }

        self.add_token(Token::EOF);
        Ok(())
    }

    fn add_token(&mut self, token: Token) {
        if token != Token::Skip {
            self.tokens.push(token);
        }
    }

    fn get_tokens(&mut self, str_iter: &mut Peekable<Chars<'_>>) -> Option<TokenHandler> {
        while let Some(ch) = str_iter.peek() {
            println!("{}", ch);
            let token = match ch {
                ch if ch.is_whitespace() => _next!(str_iter, Token::Skip),
                '\n' | ';' => {
                    self.line += 1;
                    self.column = 0;
                    _next!(str_iter, Token::LineDelim)
                }
                '(' => _next!(str_iter, Token::LeftParen),
                ')' => _next!(str_iter, Token::RightParen),
                '/' => match Self::parse_slash(&mut self.carry_string, Some(str_iter)) {
                    Ok(token) => token,
                    _ => return Some(&Self::parse_slash),
                },
                '=' => match Self::parse_equal(&mut self.carry_string, Some(str_iter)) {
                    Ok(token) => token,
                    _ => return Some(&Self::parse_equal),
                },
                '"' => match Self::parse_string(&mut self.carry_string, Some(str_iter)) {
                    Ok(token) => token,
                    _ => return Some(&Self::parse_string),
                },
                _ => match Self::parse_word(&mut self.carry_string, Some(str_iter)) {
                    Ok(token) => token,
                    _ => return Some(&Self::parse_word),
                },
            };

            self.add_token(token);
        }
        None
    }

    fn parse_slash(
        carry_string: &mut String,
        str_iter: Option<&mut Peekable<Chars<'_>>>,
    ) -> Result<Token> {
        Ok(if let Some(str_iter) = str_iter {
            if !carry_string.is_empty() {
                match str_iter.find(|c| *c == '\n') {
                    Some(_) => {
                        carry_string.clear();
                        return Ok(Token::Skip);
                    }
                    None => return Err(ErrorKind::UnexpectedEof.into()),
                }
            }

            match str_iter.peek() {
                Some('/') => Token::Slash,
                None => return Err(ErrorKind::UnexpectedEof.into()),
                _ => Token::Slash,
            }
        } else {
            Token::Slash
        })
    }

    fn parse_equal(
        _carry_string: &mut String,
        str_iter: Option<&mut Peekable<Chars<'_>>>,
    ) -> Result<Token> {
        // carry_string.clear();
        Ok(if let Some(str_iter) = str_iter {
            str_iter.next().unwrap();
            match str_iter.peek() {
                Some('=') => Token::DoubleEqual,
                None => return Err(ErrorKind::UnexpectedEof.into()),
                _ => Token::Equal,
            }
        } else {
            Token::Equal
        })
    }

    fn parse_string(
        carry_string: &mut String,
        str_iter: Option<&mut Peekable<Chars<'_>>>,
    ) -> Result<Token> {
        if let Some(str_iter) = str_iter {
            if carry_string.is_empty() {
                str_iter.next().unwrap();
            }

            loop {
                match str_iter.peek() {
                    Some('\"') => break,
                    Some(_) => carry_string.push(str_iter.next().unwrap()),
                    None => return Err(ErrorKind::UnexpectedEof.into()),
                }
            }
            str_iter.next();
        } else {
            return Err(ErrorKind::UnexpectedEof.into());
        }

        Ok(Token::Str(
            carry_string.drain(..carry_string.len()).collect(),
        ))
    }

    fn parse_word(
        carry_string: &mut String,
        str_iter: Option<&mut Peekable<Chars<'_>>>,
    ) -> Result<Token> {
        if let Some(str_iter) = str_iter {
            // carry_string.push(str_iter.next().unwrap());

            // let is_valid_char = |ch: char| ch.is_ascii_alphanumeric() || ch == '_';
            let is_valid_char = |ch: char| !ch.is_whitespace() && !ch.is_ascii_punctuation();
            loop {
                match str_iter.peek() {
                    Some(ch) if is_valid_char(*ch) => {
                        // println!("next word: {}", *ch);
                        carry_string.push(str_iter.next().unwrap());
                    }
                    Some(_) => break,
                    None => return Err(ErrorKind::UnexpectedEof.into()),
                }
            }
        }

        let res: String = carry_string.drain(..carry_string.len()).collect();
        Ok(match Token::match_lexem(res.as_ref()) {
            Some(token) => token.clone(),
            _ => Token::Ident(res),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Result;

    fn vectors_equal(a: Vec<Token>, b: Vec<Token>) -> bool {
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
        lexer.parse(file)?;

        let expected = vec![
            Token::Let,
            Token::Ident("b".to_string()),
            Token::Equal,
            Token::Str("Send you my love 😘".to_string()),
            Token::LineDelim,
            Token::EOF,
        ];
        println!("{:?}", lexer.tokens);
        assert!(vectors_equal(expected, lexer.tokens));

        Ok(())
    }

    // #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Kind(UnexpectedEof)")]
    fn test_uncomplete_string() {
        let mut lexer = Lexer::new();
        let test_string: &[u8] = "type = \"human".as_bytes();
        lexer.parse(test_string).unwrap();
    }

    // #[test]
    fn test_utf8_string() {
        let mut lexer = Lexer::new();
        let test_string: &[u8] = "💘💘 \"YOU LOOK SO CHARMING TODAY\" 💘💘".as_bytes();
        lexer.parse(test_string).unwrap();
        let expected = vec![
            Token::Ident("💘💘".to_string()),
            Token::Str("YOU LOOK SO CHARMING TODAY".to_string()),
            Token::Ident("💘💘".to_string()),
            Token::EOF,
        ];
        // println!("{:?}", lexer.tokens);
        assert!(vectors_equal(expected, lexer.tokens));
    }
}
