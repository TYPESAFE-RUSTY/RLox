use super::expr::{evaluate, Expr};

#[derive(Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
}

impl Stmt {
    fn visit(&self) {
        match self {
            Stmt::Expression { expression } => {
                let _ = evaluate(expression);
            }
            Stmt::Print { expression } => {
                let val = evaluate(expression);
                println!("{}", val);
            }
        }
    }
}

pub fn execute(statement: Stmt) {
    statement.visit();
}
