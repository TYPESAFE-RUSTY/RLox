use std::{
    cmp::Ordering,
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone)]
pub enum Object {
    IntValue(i64),
    FloatValue(f64),
    StringValue(String),
    // _Identifier(String),
    True,
    False,
    Null,
}

// overloading ==
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::False, Object::False) => true,
            (Object::True, Object::True) => true,
            (Object::Null, Object::Null) => true,
            (Object::IntValue(val), Object::IntValue(other)) => val == other,
            (Object::IntValue(val), Object::FloatValue(other)) => *val as f64 == *other,
            (Object::FloatValue(val), Object::FloatValue(other)) => val == other,
            (Object::FloatValue(val), Object::IntValue(other)) => *val == *other as f64,
            (Object::StringValue(val), Object::StringValue(other)) => val == other,
            _ => false,
        }
    }
}

// overloading <,<=,>,>=
impl PartialOrd for Object {
    fn ge(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Object::IntValue(val), Object::IntValue(rhs)) => val >= rhs,
            (Object::IntValue(val), Object::FloatValue(rhs)) => *val as f64 >= *rhs,
            (Object::FloatValue(val), Object::FloatValue(rhs)) => val >= rhs,
            (Object::FloatValue(val), Object::IntValue(rhs)) => *val >= *rhs as f64,
            _ => false,
        }
    }

    fn gt(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Object::IntValue(val), Object::IntValue(rhs)) => val > rhs,
            (Object::IntValue(val), Object::FloatValue(rhs)) => *val as f64 > *rhs,
            (Object::FloatValue(val), Object::FloatValue(rhs)) => val > rhs,
            (Object::FloatValue(val), Object::IntValue(rhs)) => *val > *rhs as f64,
            _ => false,
        }
    }

    fn le(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Object::IntValue(val), Object::IntValue(rhs)) => val <= rhs,
            (Object::IntValue(val), Object::FloatValue(rhs)) => *val as f64 <= *rhs,
            (Object::FloatValue(val), Object::FloatValue(rhs)) => val <= rhs,
            (Object::FloatValue(val), Object::IntValue(rhs)) => *val <= *rhs as f64,
            _ => false,
        }
    }

    fn lt(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Object::IntValue(val), Object::IntValue(rhs)) => val < rhs,
            (Object::IntValue(val), Object::FloatValue(rhs)) => (*val as f64) < *rhs,
            (Object::FloatValue(val), Object::FloatValue(rhs)) => val < rhs,
            (Object::FloatValue(val), Object::IntValue(rhs)) => *val < *rhs as f64,
            _ => false,
        }
    }

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Object::IntValue(val1), Object::IntValue(val2)) => val1.partial_cmp(val2),
            (Object::IntValue(val), Object::FloatValue(rhs)) => (*val as f64).partial_cmp(rhs),
            (Object::FloatValue(val), Object::IntValue(rhs)) => val.partial_cmp(&(*rhs as f64)),
            (Object::FloatValue(val1), Object::FloatValue(val2)) => val1.partial_cmp(val2),
            _ => None, // Handle other cases (e.g., different types) as needed
        }
    }
}

// overloading -
impl Sub for Object {
    type Output = Object;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Object::IntValue(val) => match rhs {
                Object::IntValue(rhs) => Object::IntValue(val - rhs),
                Object::FloatValue(rhs) => Object::FloatValue(val as f64 - rhs),
                _ => Object::Null,
            },
            Object::FloatValue(val) => match rhs {
                Object::FloatValue(rhs) => Object::FloatValue(val - rhs),
                Object::IntValue(rhs) => Object::FloatValue(val - rhs as f64),
                _ => Object::Null,
            },
            _ => Object::Null,
        }
    }
}

// overloading +
impl Add for Object {
    type Output = Object;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Object::IntValue(lhs), Object::IntValue(rhs)) => Object::IntValue(lhs + rhs),
            (Object::IntValue(lhs), Object::FloatValue(rhs)) => {
                Object::FloatValue(lhs as f64 + rhs)
            }
            (Object::FloatValue(lhs), Object::IntValue(rhs)) => {
                Object::FloatValue(lhs + rhs as f64)
            }
            (Object::FloatValue(lhs), Object::FloatValue(rhs)) => Object::FloatValue(lhs + rhs),
            (Object::StringValue(lhs), Object::StringValue(rhs)) => Object::StringValue(lhs + &rhs),
            _ => Object::Null,
        }
    }
}

// overloading *
impl Mul for Object {
    type Output = Object;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Object::IntValue(val) => match rhs {
                Object::IntValue(rhs) => Object::IntValue(val * rhs),
                Object::FloatValue(rhs) => Object::FloatValue(val as f64 * rhs),
                _ => Object::Null,
            },
            Object::FloatValue(val) => match rhs {
                Object::FloatValue(rhs) => Object::FloatValue(val * rhs),
                Object::IntValue(rhs) => Object::FloatValue(val * rhs as f64),
                _ => Object::Null,
            },
            _ => Object::Null,
        }
    }
}

// overloading /
impl Div for Object {
    type Output = Object;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Object::IntValue(val) => match rhs {
                Object::IntValue(rhs) => Object::FloatValue(val as f64 / rhs as f64),
                Object::FloatValue(rhs) => Object::FloatValue(val as f64 / rhs),
                _ => Object::Null,
            },
            Object::FloatValue(val) => match rhs {
                Object::FloatValue(rhs) => Object::FloatValue(val / rhs),
                Object::IntValue(rhs) => Object::FloatValue(val / rhs as f64),
                _ => Object::Null,
            },
            _ => Object::Null,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::IntValue(value) => {
                write!(f, "{}", value)
            }
            Object::FloatValue(value) => {
                write!(f, "{}", value)
            }
            Object::StringValue(value) => {
                write!(f, "{}", value)
            }
            // Object::_Identifier(value) => {
            //     write!(f, "{}", value)
            // }
            Object::Null => {
                write!(f, "Nil")
            }
            Object::False => {
                write!(f, "False")
            }
            Object::True => {
                write!(f, "True")
            }
        }
    }
}

impl Object {
    pub fn _int(&self) -> Result<i64, &str> {
        match self {
            Object::IntValue(val) => Ok(*val),
            _ => Err("Expected int"),
        }
    }

    pub fn _float(&self) -> Result<f64, &str> {
        match self {
            Object::IntValue(val) => Ok(*val as f64),
            Object::FloatValue(val) => Ok(*val),
            _ => Err("Expected int"),
        }
    }
}
