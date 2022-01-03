use lazy_static::lazy_static;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, AstError>;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum AstError {
    LetStmt,
    Expression,
    General,
}

lazy_static! {
    static ref AST_ERROR_MAP: HashMap<AstError, &'static str> = vec![
        (AstError::LetStmt, "parsing Let statement"),
        (AstError::General, "general error")
    ]
    .into_iter()
    .collect();
}

impl ToString for AstError {
    fn to_string(&self) -> String {
        if let Some(err) = AST_ERROR_MAP.get(self) {
            return err.to_string();
        }
        AST_ERROR_MAP.get(&AstError::General).unwrap().to_string()
    }
}
