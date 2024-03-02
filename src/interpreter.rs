use crate::parser::stmt::{execute, Stmt};

pub fn interpret(statements: Vec<Stmt>) {
    for statement in statements {
        execute(statement);
    }
}
