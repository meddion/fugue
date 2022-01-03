use crate::ast::errors::{AstError, Result};
use crate::ast::{Expression, Node, Statement};
use crate::{lexer::*, token::*};
use std::rc::Rc;

pub struct LetStmt {
    ident: String,
    expr: Option<Rc<dyn Expression>>,
}

fn parse_expression(token_iter: &mut LexerIter) -> Result<Rc<dyn Expression>> {
    // while if let Some(token) = token_iter.next() {
    match token_iter.next() {
        Some(Token::Str(string)) => return Ok(StringExp::new(string.clone())),
        _ => (),
    }
    // }

    Err(AstError::Expression)
}
pub struct StringExp {
    string: String,
}
impl StringExp {
    fn new(string: String) -> Rc<dyn Expression> {
        Rc::new(StringExp { string })
    }
}

impl Node for StringExp {
    fn dtype(&self) -> &'static str {
        "StringExp"
    }
}
impl Expression for StringExp {}

impl Node for LetStmt {
    fn dtype(&self) -> &'static str {
        "Let"
    }
}
impl Statement for LetStmt {}
impl LetStmt {
    pub fn parse(tokens_iter: &mut LexerIter) -> Result<Rc<dyn Statement>> {
        if !next_is_of_type(tokens_iter, &Token::Let) {
            return Err(AstError::LetStmt);
        }
        tokens_iter.next().unwrap();

        let ident = match tokens_iter.next() {
            Some(Token::Ident(name)) => name.clone(),
            _ => return Err(AstError::LetStmt),
        };

        if !next_is_of_type(tokens_iter, &Token::Equal) {
            return Err(AstError::LetStmt);
        }
        tokens_iter.next().unwrap();

        let expr = parse_expression(tokens_iter)?;

        Ok(Rc::new(LetStmt {
            ident: ident,
            expr: Some(expr),
        }))
    }
}

// pub struct BlockExp {
//     exps: Vec<Rc<dyn Node>>,
// }

// pub struct Predicate;

// impl Node for Predicate {
//     fn dtype(&self) -> &'static str {
//         "Predicate"
//     }
// }

// impl Node for BlockExp {
//     fn dtype(&self) -> &'static str {
//         "Block Node"
//     }
// }

// pub struct IfExp {
//     exp: Rc<dyn Node>,
// }

// impl Node for IfExp {
//     fn dtype(&self) -> &'static str {
//         "If Exp"
//     }
// }
