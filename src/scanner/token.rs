use crate::scanner::object;
use std::fmt;

#[derive(Clone)]
pub struct Token {
    pub tokentype: Tokentype,
    pub lexeme: String,
    pub literal: object::Object,
    pub line: usize,
}

impl Token {
    pub fn new(
        tokentype: Tokentype,
        lexeme: String,
        literal: object::Object,
        line: usize,
    ) -> Token {
        Token {
            tokentype,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.tokentype, self.lexeme, self.literal)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Tokentype {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String,
    Number,
    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

// dead code
impl fmt::Display for Tokentype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tokentype::LeftParen => write!(f, "LeftParen"),
            Tokentype::RightParen => write!(f, "RightParen"),
            Tokentype::LeftBrace => write!(f, "LeftBrace"),
            Tokentype::RightBrace => write!(f, "RightBrace"),
            Tokentype::Comma => write!(f, "Comma"),
            Tokentype::Dot => write!(f, "Dot"),
            Tokentype::Minus => write!(f, "Minus"),
            Tokentype::Plus => write!(f, "Plus"),
            Tokentype::Semicolon => write!(f, "Semicolon"),
            Tokentype::Slash => write!(f, "Slash"),
            Tokentype::Star => write!(f, "Star"),
            Tokentype::Bang => write!(f, "Bang"),
            Tokentype::BangEqual => write!(f, "BangEqual"),
            Tokentype::Equal => write!(f, "Equal"),
            Tokentype::EqualEqual => write!(f, "EqualEqual"),
            Tokentype::Greater => write!(f, "Greater"),
            Tokentype::GreaterEqual => write!(f, "GreaterEqual"),
            Tokentype::Less => write!(f, "Less"),
            Tokentype::LessEqual => write!(f, "LessEqual"),
            Tokentype::Identifier => write!(f, "Identifier"),
            Tokentype::String => write!(f, "String"),
            Tokentype::Number => write!(f, "Number"),
            Tokentype::And => write!(f, "And"),
            Tokentype::Class => write!(f, "Class"),
            Tokentype::Else => write!(f, "Else"),
            Tokentype::False => write!(f, "False"),
            Tokentype::Fun => write!(f, "Fun"),
            Tokentype::For => write!(f, "For"),
            Tokentype::If => write!(f, "If"),
            Tokentype::Nil => write!(f, "Nil"),
            Tokentype::Or => write!(f, "Or"),
            Tokentype::Print => write!(f, "Print"),
            Tokentype::Return => write!(f, "Return"),
            Tokentype::Super => write!(f, "Super"),
            Tokentype::This => write!(f, "This"),
            Tokentype::True => write!(f, "True"),
            Tokentype::Var => write!(f, "Var"),
            Tokentype::While => write!(f, "While"),
            Tokentype::Eof => write!(f, "Eof"),
        }
    }
}
