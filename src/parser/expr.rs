use crate::scanner::{object::Object, token::Token};
use std::fmt;

// following enum implements store for this cfg
// expression → literal  | unary  | binary  | grouping ;
// literal → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping → "(" expression ")" ;
// unary → ( "-" | "!" ) expression ;
// binary → expression operator expression ;
// operator → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+" | "-" | "*" | "/" ;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Object,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
}

// needed for debug later if i am not lucky
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator, //Operator from above grammar
                right,
            } => {
                write!(f, "({} {} {})", operator, left, right)
            }
            Expr::Grouping { expression } => {
                write!(f, "(group {})", expression)
            }
            Expr::Literal { value } => {
                write!(f, "{}", value)
            }
            Expr::Unary { operator, right } => {
                write!(f, "({} {})", operator, right)
            }
            Expr::Variable { name } => write!(f, "(variable : {})", name),
            Expr::Assign { name, value } => write!(f, "(Assignment : {} = {})", name, value),
        }
    }
}

// variation from lox i want int 0 ,float 0 and strlen 0 to be false and everything else to be true
pub fn is_truthy(object: &Object) -> Object {
    match object {
        Object::Null => Object::False,
        Object::True => Object::True,
        Object::False => Object::False,
        Object::IntValue(value) => {
            if *value == 0 {
                Object::False
            } else {
                Object::True
            }
        }
        Object::FloatValue(value) => {
            if *value == 0.0 {
                Object::False
            } else {
                Object::True
            }
        }
        Object::StringValue(str) => {
            if str.len() == 0 {
                Object::False
            } else {
                Object::True
            }
        }
    }
}

// checks if both operands are numbers
fn _bin_operand_number(left: &Object, right: &Object) -> bool {
    (matches!(left, Object::IntValue(_)) || matches!(left, Object::FloatValue(_)))
        && (matches!(right, Object::IntValue(_)) || matches!(right, Object::FloatValue(_)))
}

// utility to convert boolean to Object::bool
pub fn bool(val: bool) -> Object {
    if val {
        Object::True
    } else {
        Object::False
    }
}
