pub mod errors;
pub mod nodes;
use crate::ast::errors::{AstError, Result};
use crate::ast::nodes::*;
use crate::{lexer::*, token::*};
use std::rc::Rc;

pub trait Node {
    fn dtype(&self) -> &'static str;
}
pub trait Expression: Node {}
pub trait Statement: Node {}

pub struct Program {
    statements: Vec<Rc<dyn Statement>>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn create_ast(&mut self, token_iter: &mut LexerIter) -> Result<()> {
        while let Some(token) = token_iter.peek() {
            let stmt = match token {
                Token::Let => LetStmt::parse(token_iter),
                Token::EOF | Token::LineDelim => {
                    token_iter.next().unwrap();
                    continue;
                }
                _ => return Err(AstError::General),
            };
            self.statements.push(stmt?);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ast_test() -> Result<()> {
        let mut parser = Program::new();

        let mut lexer = Lexer::new();

        lexer.tokens = vec![
            Token::Let,
            Token::Ident("cat_name".to_owned()),
            Token::Equal,
            Token::Str("Tomas".to_owned()),
            Token::LineDelim,
        ];

        parser.create_ast(&mut lexer.into_iter())?;
        // lexer.clear();

        Ok(())
    }
}
