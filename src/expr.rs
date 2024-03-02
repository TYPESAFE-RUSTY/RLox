use crate::token::{Object::Object, Token, Tokentype};
use std::fmt;

// following enum implements store for this cfg
// expression → literal  | unary  | binary  | grouping ;
// literal → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping → "(" expression ")" ;
// unary → ( "-" | "!" ) expression ;
// binary → expression operator expression ;
// operator → "==" | "!=" | "<" | "<=" | ">" | ">=" | "+" | "-" | "*" | "/" ;

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
        }
    }
}

// Evaluating expressions
impl Expr {
    pub fn interpret(&self) {
        let value: Object = evaluate(self);
        println!("{}", value);
    }

    fn visit(&self) -> Object {
        match self {
            Expr::Literal { value } => {
                let val = value;
                val.clone()
            }
            Expr::Grouping { expression } => evaluate(&expression),
            Expr::Unary { operator, right } => {
                let right = evaluate(&right);

                match operator.tokentype {
                    Tokentype::Bang => {
                        let truthy = is_truthy(&right);
                        if truthy == Object::True {
                            Object::False
                        } else {
                            Object::True
                        }
                    }
                    Tokentype::Minus => match right {
                        Object::IntValue(value) => Object::IntValue(-value),
                        Object::FloatValue(value) => Object::FloatValue(-value),
                        _ => Object::Null,
                    },
                    _ => Object::Null,
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = evaluate(&left);
                let right = evaluate(&right);
                // matchception begins here ;) good luck understanding code
                // changed my mind writing clean code ;)
                match operator.tokentype {
                    Tokentype::Minus => left - right,
                    Tokentype::Plus => left + right,
                    Tokentype::Slash => left / right,
                    Tokentype::Star => left * right,
                    Tokentype::Greater => bool(left > right),
                    Tokentype::GreaterEqual => bool(left >= right),
                    Tokentype::Less => bool(left < right),
                    Tokentype::LessEqual => bool(left <= right),
                    Tokentype::EqualEqual => bool(left == right),
                    Tokentype::BangEqual => bool(left != right),
                    _ => Object::Null,
                }
            }
        }
    }
}

fn evaluate(expr: &Expr) -> Object {
    expr.visit()
}

// variation from lox i want int 0 ,float 0 and strlen 0 to be false and everything else to be true
fn is_truthy(object: &Object) -> Object {
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
fn bin_operand_number(left: &Object, right: &Object) -> bool {
    (matches!(left, Object::IntValue(_)) || matches!(left, Object::FloatValue(_)))
        && (matches!(right, Object::IntValue(_)) || matches!(right, Object::FloatValue(_)))
}

// utility to convert boolean to Object::bool
fn bool(val: bool) -> Object {
    if val {
        Object::True
    } else {
        Object::False
    }
}
