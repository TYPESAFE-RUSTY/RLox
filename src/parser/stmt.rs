use crate::token::Token;

use super::expr::Expr;

#[derive(Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initalizer: Expr },
    Block { statements: Vec<Stmt> },
}
