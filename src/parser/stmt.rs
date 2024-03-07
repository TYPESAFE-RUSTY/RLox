use crate::token::Token;

use super::expr::Expr;

#[derive(Clone)]
pub enum Stmt {
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initalizer: Expr,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Box<Option<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Block {
        statements: Vec<Stmt>,
    },
}
