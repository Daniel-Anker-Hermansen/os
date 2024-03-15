use std::iter::Peekable;

use crate::{ast::*, lexer::{self, Location, Token}};

type Lexer<'src> = Peekable<lexer::Lexer<'src>>;

pub fn parse_module<'src>(lexer: &mut Lexer<'src>) -> Module<'src> {
    let mut items = Vec::new();
    while lexer.peek().is_some() {
        items.push(parse_item(lexer).unwrap());
    }
    Module {
        items,
    }
}

fn parse_item<'src>(lexer: &mut Lexer<'src>) -> Option<Item<'src>> {
    let token_with_location = lexer.next()?;
    let item = match token_with_location.token {
        lexer::Token::Fn => {
            let name = expected_some("identifier", parse_identifier(lexer));
            eat(lexer, Token::LeftParen);
            let signature = parse_signature(lexer);
            eat(lexer, Token::RightParen);

            Item::Function(Function {
                name,
                signature,
                body: todo!(),
            })
        },
        lexer::Token::Interupt => todo!(),
        lexer::Token::Intrinsic => todo!(),
        lexer::Token::Struct => todo!(),
        _ => expected("function, interupt, instrinsic or struct", token_with_location.start),
    };
    Some(item)
}

fn parse_identifier<'src>(lexer: &mut Lexer<'src>) -> Option<&'src str> {
    if let Some(Token::Identifier(identifier)) = lexer.next().map(|t| t.token) {
        return Some(identifier);
    }
    None
}

fn parse_signature<'src>(lexer: &mut Lexer<'src>) -> Signature<'src> {

    unimplemented!()
}

fn eat(lexer: &mut Lexer, expected_: Token) {
    if let Some(actual) = lexer.next() {
        if actual.token == expected_ {
            return;
        }
    }    
    expected("fill in later", Location::new());
}

fn expected_some<T>(msg: &str, val: Option<T>) -> T {
    match val {
        Some(v) => v,
        None => expected(msg, Location::new()),
    }
}

fn expected(msg: &str, location: Location) -> ! {
    println!("expected {} at line {} columm {}", msg, location.line, location.col); 
    std::process::exit(1);
}
