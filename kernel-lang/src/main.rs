use std::{convert::identity, iter::repeat_with};

use lexer::TokenWithLocation;

mod ast;
mod lexer;
mod parser;

fn main() {
    let src = std::fs::read_to_string(&std::env::args().nth(1).unwrap()).unwrap();
    let mut lexer = lexer::Lexer::new(&src);
    let module = parser::parse_module(&mut lexer.peekable());
}
