use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token<'static>> = {
        [
            ("fn", Token::Fn),
            ("interupt", Token::Interupt),
            ("intrinsic", Token::Intrinsic),
            ("mod", Token::Mod),
            ("if", Token::If),
            ("else", Token::Else),
            ("loop", Token::Loop),
            ("break", Token::Break),
            ("continue", Token::Continue),
            ("struct", Token::Struct),
            ("trait", Token::Trait),
            ("let", Token::Let),
        ]
        .into_iter()
        .collect()
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token<'src> {
    // Keywords
    Fn,
    Interupt,
    Intrinsic,
    Mod,
    If,
    Else,
    Loop,
    Break,
    Continue,
    Struct,
    Trait,
    Let,

    // Symbols
    Plus,
    Minus,
    Star,
    Division,
    ShiftLeft,
    ShiftRight,
    Greater,
    Less,
    And,
    Or,
    Equal,
    NotEqual,
    Not,
    Colon,
    Semicolon,
    Arrow,
    LeftParen,
    RightParen,
    LeftSquirly,
    RightSquirly,

    // Literals
    NumericLiteral(&'src str),

    // Identifiers
    Identifier(&'src str),
}

fn token_from_literal_string<'src>(literal: &'src str) -> Token {
    KEYWORDS
        .get(literal)
        .cloned()
        .unwrap_or_else(|| Token::Identifier(literal))
}

#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub line: usize,
    pub col: usize,
}

impl Location {
    pub fn new() -> Location {
        Location { line: 1, col: 1 }
    }

    fn add_char(self, ch: char) -> Self {
        Location {
            line: self.line + (ch == '\n') as usize,
            col: self.col * (ch != '\n') as usize + 1,
        }
    }
}

#[derive(Debug)]
pub struct TokenWithLocation<'src> {
    pub token: Token<'src>,
    pub start: Location,
    pub end: Location,
}

pub struct Lexer<'src> {
    src: &'src str,
    index: usize,
    position: Location,
}

impl<'src> Lexer<'src> {
    pub fn new(src: &'src str) -> Lexer<'src> {
        Lexer {
            src,
            index: 0,
            position: Location::new(),
        }
    }

    fn peek(&self) -> Option<char> {
        self.src[self.index..].chars().next()
    }

    fn pop(&mut self) -> Option<char> {
        self.peek().inspect(|ch| self.index += ch.len_utf8())
    }

    fn next(&mut self) -> Option<TokenWithLocation<'src>> {
        while self.peek().map(|ch| ch.is_whitespace()).unwrap_or(false) {
            self.position = self.position.add_char(self.pop().expect("is some because of unwrap_or above"));
        }
        let next = self.peek()?;
        let start_index = self.index;
        let token = if let Some(token) = self.symbolic(next) {
            token   
        }
        else if next.is_numeric() {
            while self.peek().map(|ch| ch.is_numeric()).unwrap_or(false) {
                self.pop();
            }
            Token::NumericLiteral(&self.src[start_index..self.index])
        }
        else if next.is_alphabetic() || next == '_' {
            while self.peek().map(|ch| ch.is_numeric() || ch.is_alphabetic() || ch == '_').unwrap_or(false) {
                self.pop();
            }
            token_from_literal_string(&self.src[start_index..self.index])
        }
        else {
            panic!("invalid charachter");
        };
        let start = self.position;
        self.position = self.src[start_index..self.index].chars().fold(self.position, Location::add_char);
        let end = self.position;
        Some(TokenWithLocation {
            token,
            start,
            end,
        })
    }

    fn symbolic(&mut self, ch: char) -> Option<Token<'static>> {
        let (token, size) = match ch {
            '+' => (Token::Plus, 1),
            '-' => {
                self.pop();
                match self.peek() {
                    Some('>') => (Token::Arrow, 1),
                    _ => (Token::Minus, 0)
                }
            }
            '*' => (Token::Star, 1),
            '/' => (Token::Division, 1),
            '>' => {
                match self.peek() {
                    Some('>') => (Token::ShiftRight, 2),
                    _ => (Token::Greater, 1)
                }
            }
            '<' => {
                match self.peek() {
                    Some('<') => (Token::ShiftLeft , 2),
                    _ => (Token::Less, 1)
                }
            }
            '&' => (Token::And, 1),
            '|' => (Token::Or, 1),
            '=' => (Token::Equal, 1),
            '!' => {
                match self.peek() {
                    Some('=') => (Token::NotEqual, 2),
                    _ => (Token::Not, 1)
                }
            }
            ':' => (Token::Colon, 1),
            ';' => (Token::Semicolon, 1),
            '(' => (Token::LeftParen, 1),
            ')' => (Token::RightParen, 1),
            '{' => (Token::LeftSquirly, 1),
            '}' => (Token::RightSquirly, 1),
            _ => return None,
        };
        self.index += size;
        Some(token)
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = TokenWithLocation<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
