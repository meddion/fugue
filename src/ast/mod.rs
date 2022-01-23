pub mod errors;
use crate::ast::errors::{AstError, Result};
use crate::{
    lexer::TokenStream,
    tokens::{next_is_of_type, Token},
};

#[derive(Debug)]
enum BinaryOp {
    Plus,
    Minus,
    Div,
    Mult,

    Eq,
    Nq,
    More { is_eq: bool },
    Less { is_eq: bool },
}

#[derive(Debug)]
enum UnaryOp {
    Plus,
    Minus,
}

#[derive(Debug)]
struct Binary {
    left: Box<Expression>,
    right: Box<Expression>,
    op: BinaryOp,
}

impl Binary {
    fn new(left: Expression, right: Expression, op: BinaryOp) -> Binary {
        Binary {
            left: Box::new(left),
            right: Box::new(right),
            op: op,
        }
    }
}

impl Into<Expression> for Binary {
    fn into(self) -> Expression {
        Expression::Binary(Binary {
            left: self.left,
            right: self.right,
            op: self.op,
        })
    }
}

#[derive(Debug)]
pub enum Expression {
    Num(f64),
    Str(String),
    Bool(bool),
    Var(String),

    Binary(Binary),

    Unary { exp: Box<Expression>, op: UnaryOp },
    Grouping(Vec<Expression>),
}

#[derive(Debug)]
pub enum Statement {
    Let {
        ident: String,
        exp: Expression,
    },
    Loop {
        pred: Expression,
        block: Vec<Statement>,
    },
}

#[derive(Debug)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn create_ast(token_iter: &mut TokenStream) -> Result<Program> {
        Ok(Program {
            statements: Self::parse(token_iter)?,
        })
    }

    fn parse(token_iter: &mut TokenStream) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();
        while let Some(token) = token_iter.peek() {
            let stmt = match token {
                Token::Let => Self::parse_stmt_let(token_iter),
                Token::EOF | Token::LineDelim => {
                    token_iter.next().unwrap();
                    continue;
                }
                _ => return Err(AstError::General),
            };

            statements.push(stmt?);
        }

        Ok(statements)
    }

    fn parse_expression(token_iter: &mut TokenStream) -> Result<Expression> {
        while if let Some(Some(token)) = token_iter.next() {
            match token {
               Token::Str(string) => return Ok(Expression::Str(string.clone())),
                _ => (),
            }

        }

        Err(AstError::Expression)
    }

    fn parse_stmt_let(tokens_iter: &mut TokenStream) -> Result<Statement> {
        if !next_is_of_type(tokens_iter, Token::Let) {
            return Err(AstError::LetStmt);
        }
        tokens_iter.next().unwrap();

        let ident = match tokens_iter.next() {
            Some(Token::Ident(name)) => name.clone(),
            _ => return Err(AstError::LetStmt),
        };

        if !next_is_of_type(tokens_iter, Token::Equal) {
            return Err(AstError::LetStmt);
        }
        tokens_iter.next().unwrap();

        let expr = Self::parse_expression(tokens_iter)?;

        Ok(Statement::Let {
            ident: ident,
            exp: expr,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn ast_test() -> Result<()> {
        // let mut parser = Program::new();

        // parser.statements = vec![
        //     Statement::For {
        //         pred: Binary::new(
        //             Expression::Var("num".to_owned()),
        //             Expression::Num(10.0),
        //             BinaryOp::More { is_eq: false },
        //         )
        //         .into(),

        //         block: vec![Statement::Let {
        //             ident: "is_happy".to_owned(),
        //             exp: Some(Expression::Bool(true)),
        //         }],
        //     },
        //     Statement::Let {
        //         ident: "greeting".to_owned(),
        //         exp: Some(Expression::Str("Hello!!!".to_owned())),
        //     },
        // ];

        // println!("{:?}", parser.statements);

        // let mut lexer = Lexer::new();

        // lexer.tokens = vec![
        //     Token::Let,
        //     Token::Ident("cat_name".to_owned()),
        //     Token::Equal,
        //     Token::Str("Tomas".to_owned()),
        //     Token::LineDelim,
        // ];

        // parser.create_ast(&mut lexer.into_iter())?;
        // lexer.clear();

        Ok(())
    }
}
