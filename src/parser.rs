use core::panic;
pub mod expr;
pub mod stmt;
use crate::scanner::{
    object::Object,
    token::{Token, Tokentype},
};
use expr::Expr;
use stmt::Stmt;

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
    statements: Vec<Stmt>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
            statements: Vec::new(),
        }
    }
}

// this impl implements grammar written in Expr.rs
impl Parser {
    pub fn parse(&mut self) -> Vec<Stmt> {
        while !self.is_at_end() {
            let statement = self.declaration();
            match statement {
                Ok(val) => self.statements.push(val),
                Err(msg) => {
                    println!("{}", msg);
                    self.synchronize();
                }
            }
        }
        self.statements.clone()
    }

    fn declaration(&mut self) -> Result<Stmt, &str> {
        if self.match_tokens(&[Tokentype::Var]) {
            self.var_declaration()
        } else {
            Ok(self.statement())
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, &str> {
        let name = self.consume(Tokentype::Identifier, "Expect variable name");

        match name {
            Ok(val) => {
                let name = val;
                let initalizer: Expr;
                if self.match_tokens(&[Tokentype::Equal]) {
                    initalizer = self.expression();
                } else {
                    initalizer = Expr::Literal {
                        value: Object::Null,
                    };
                }
                let _ = self.consume(
                    Tokentype::Semicolon,
                    "Expect ';' after variable declaration",
                );
                Ok(Stmt::Var { name, initalizer })
            }
            Err(msg) => {
                println!("{}", msg);
                Err("Parsing error occured")
            }
        }
    }

    fn statement(&mut self) -> Stmt {
        if self.match_tokens(&[Tokentype::Print]) {
            self.print_statement()
        } else if self.match_tokens(&[Tokentype::LeftBrace]) {
            Stmt::Block {
                statements: self.block(),
            }
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(Tokentype::RightBrace) && !self.is_at_end() {
            match self.declaration() {
                Ok(val) => statements.push(val),
                Err(msg) => println!("{}", msg),
            }
        }

        let _ = self.consume(Tokentype::RightBrace, "Error : Expect '}' after block.");
        statements
    }

    fn print_statement(&mut self) -> Stmt {
        let value = self.expression();
        let _ = self.consume(Tokentype::Semicolon, "Expect ';' after value.");
        Stmt::Print { expression: value }
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        let _ = self.consume(Tokentype::Semicolon, "Expect ';' after expression");
        Stmt::Expression { expression: expr }
    }

    fn expression(&mut self) -> Expr {
        match self.assignment() {
            Ok(val) => val,
            Err(msg) => {
                println!("{}", msg);
                Expr::Literal {
                    value: Object::Null,
                }
            }
        }
    }

    fn assignment(&mut self) -> Result<Expr, &str> {
        let expr = self.equality();

        if self.match_tokens(&[Tokentype::Equal]) {
            let _equals = self.previous();
            let value = self.assignment()?;

            match expr {
                Expr::Variable { name } => Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                }),
                _ => Err("Error : Invalid assignment target."),
            }
        } else {
            Ok(expr)
        }
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();

        while self.match_tokens(&[Tokentype::BangEqual, Tokentype::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_tokens(&[
            Tokentype::Greater,
            Tokentype::GreaterEqual,
            Tokentype::Less,
            Tokentype::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[Tokentype::Minus, Tokentype::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.match_tokens(&[Tokentype::Slash, Tokentype::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[Tokentype::Bang, Tokentype::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[Tokentype::False]) {
            return Expr::Literal {
                value: Object::False,
            };
        };
        if self.match_tokens(&[Tokentype::True]) {
            return Expr::Literal {
                value: Object::True,
            };
        }

        if self.match_tokens(&[Tokentype::Nil]) {
            return Expr::Literal {
                value: Object::Null,
            };
        }

        if self.match_tokens(&[Tokentype::Number, Tokentype::String]) {
            return Expr::Literal {
                value: self.previous().literal,
            };
        }
        if self.match_tokens(&[Tokentype::Identifier]) {
            return Expr::Variable {
                name: self.previous(),
            };
        }
        if self.match_tokens(&[Tokentype::LeftParen]) {
            let expr = self.expression();
            let _ = self.consume(Tokentype::RightParen, "Expect ')' after expression ");
            Expr::Grouping {
                expression: Box::new(expr),
            }
        } else {
            error(self.peek(), "Expect expression");
            panic!("yo wtf");
        }
    }
}

impl Parser {
    // checks if the current token matches any of the token types provided
    fn match_tokens(&mut self, types: &[Tokentype]) -> bool {
        for &token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token: Tokentype) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().tokentype == token
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        let token = self.previous();
        token
    }

    fn is_at_end(&self) -> bool {
        self.peek().tokentype == Tokentype::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, ty: Tokentype, message: &str) -> Result<Token, &str> {
        if self.check(ty) {
            Ok(self.advance())
        } else {
            error(self.peek(), message);
            Err("some error occured check logs")
        }
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().tokentype == Tokentype::Semicolon {
                return;
            }

            match self.peek().tokentype {
                Tokentype::Class => return,
                Tokentype::Fun => return,
                Tokentype::Var => return,
                Tokentype::For => return,
                Tokentype::If => return,
                Tokentype::While => return,
                Tokentype::Print => return,
                Tokentype::Return => return,
                _ => (),
            }
            self.advance();
        }
    }
}

fn error(token: Token, message: &str) {
    if token.tokentype == Tokentype::Eof {
        println!("{} at end {}", token.line, message)
    } else {
        println!("{} at {} ' {}", token.line, token.lexeme, message)
    }
}
