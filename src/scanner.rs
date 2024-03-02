pub mod object;
pub mod token;
use object::Object;
use std::collections::HashMap;
use token::{Token, Tokentype};

pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, Tokentype>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let map: HashMap<String, Tokentype> = HashMap::from([
            (String::from("and"), Tokentype::And),
            (String::from("class"), Tokentype::Class),
            (String::from("else"), Tokentype::Else),
            (String::from("false"), Tokentype::False),
            (String::from("for"), Tokentype::For),
            (String::from("fun"), Tokentype::Fun),
            (String::from("if"), Tokentype::If),
            (String::from("nil"), Tokentype::Nil),
            (String::from("or"), Tokentype::Or),
            (String::from("print"), Tokentype::Print),
            (String::from("return"), Tokentype::Return),
            (String::from("super"), Tokentype::Super),
            (String::from("this"), Tokentype::This),
            (String::from("true"), Tokentype::True),
            (String::from("var"), Tokentype::Var),
            (String::from("while"), Tokentype::While),
        ]);

        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: map,
        }
    }
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            Tokentype::Eof,
            String::from(""),
            Object::Null,
            self.line,
        ));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let character = self.advance();
        match character {
            // single character tokens
            '(' => self.add_token(Tokentype::LeftParen),
            ')' => self.add_token(Tokentype::RightParen),
            '{' => self.add_token(Tokentype::LeftBrace),
            '}' => self.add_token(Tokentype::RightBrace),
            ',' => self.add_token(Tokentype::Comma),
            '.' => self.add_token(Tokentype::Dot),
            '-' => self.add_token(Tokentype::Minus),
            '+' => self.add_token(Tokentype::Plus),
            ';' => self.add_token(Tokentype::Semicolon),
            '*' => self.add_token(Tokentype::Star),
            // multi character tokens
            '/' => {
                if self.check_next_char("/") {
                    // deal with comments
                    let mut comment = String::from("");
                    while self.peek() != "\n" && !self.is_at_end() {
                        let char = self.advance();
                        comment.push(char);
                    }
                    println!("comment is : {}", comment)
                } else if self.check_next_char("*") {
                    self.multi_line_comment();
                } else {
                    self.add_token(Tokentype::Slash)
                }
            }
            '!' => self.check_next_char_and_add_token("=", Tokentype::Bang, Tokentype::BangEqual),
            '=' => self.check_next_char_and_add_token("=", Tokentype::Equal, Tokentype::EqualEqual),
            '<' => self.check_next_char_and_add_token("=", Tokentype::Less, Tokentype::LessEqual),
            '>' => {
                self.check_next_char_and_add_token("=", Tokentype::Greater, Tokentype::GreaterEqual)
            }
            // token with literals
            '"' => self.string(),
            // whitespace characters
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            _ => {
                if is_digit(character) {
                    self.number()
                } else if is_alpha(character) {
                    self.identifier();
                } else {
                    println!("Unexpected token.")
                }
            }
        }
    }
}

// helper functions
impl Scanner {
    // checks if code ended
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // moves 1 character ahead in source and returns current character
    fn advance(&mut self) -> char {
        self.current += 1;
        match self.source.chars().nth(self.current - 1) {
            Some(character) => character,
            None => '\0',
        }
    }

    // adds the token provided to tokens list
    fn add_token(&mut self, tokentype: Tokentype) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(tokentype, text, Object::Null, self.line);
        self.tokens.push(token);
    }

    // adds token with its literal to the list
    fn add_token_with_literal(&mut self, tokentype: Tokentype, literal: Object) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(tokentype, text, literal, self.line);
        self.tokens.push(token);
    }

    // checks next character before adding token to the tokens list if next character dosent matches check small token is added else large token is added
    fn check_next_char_and_add_token(&mut self, check: &str, small: Tokentype, large: Tokentype) {
        if self.check_next_char(check) {
            self.add_token(large);
        } else {
            self.add_token(small)
        }
    }

    // checks next character
    fn check_next_char(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        };
        if self.source.chars().nth(self.current).unwrap().to_string() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    // checks next character and return next character (does not moves the scanner to the next character)
    fn peek(&self) -> String {
        if self.is_at_end() {
            return String::from("\0");
        }
        self.source.chars().nth(self.current).unwrap().to_string()
    }

    // just check next character dont consume
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    // utility to find string literals in source
    fn string(&mut self) {
        while self.peek().chars().nth(0).unwrap() != '"' && !self.is_at_end() {
            if self.peek() == "\n" {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            println!("Unterminated String starting at line {}", self.line);
            return;
        }

        // we are at ending "
        self.advance();

        // adding token
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(Tokentype::String, Object::StringValue(value));
    }

    fn multi_line_comment(&mut self) {
        while self.peek() != "*" && self.peek_next() != '/' && !self.is_at_end() {
            if self.peek() == "\n" {
                self.line += 1;
            }
            self.advance();
        }
        self.advance(); // at last *
        self.advance(); // consuming last /
    }

    // utility to find number literal
    fn number(&mut self) {
        while is_digit(self.peek().chars().nth(0).unwrap()) {
            self.advance();
        }
        if self.peek() == "." && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek().chars().nth(0).unwrap()) {
                self.advance();
            }
            let float = self.source[self.start..self.current]
                .parse()
                .expect("number out of range");
            self.add_token_with_literal(Tokentype::Number, Object::FloatValue(float));
            return;
        }
        // adding token
        let integer = self.source[self.start..self.current]
            .parse()
            .expect("number out of range");
        self.add_token_with_literal(Tokentype::Number, Object::IntValue(integer))
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek().chars().nth(0).unwrap()) {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].to_string();
        match self.keywords.get(&text) {
            Some(result) => self.add_token(*result),
            _ => self.add_token(Tokentype::Identifier),
        };
    }
}

// utility to check if current character is digit
pub fn is_digit(character: char) -> bool {
    character >= '0' && character <= '9'
}

pub fn is_alpha(character: char) -> bool {
    (character >= 'a' && character <= 'z')
        || (character >= 'A' && character <= 'Z')
        || character == '_'
}

pub fn is_alpha_numeric(character: char) -> bool {
    is_alpha(character) || is_digit(character)
}
