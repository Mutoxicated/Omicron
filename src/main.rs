use std::{fs::File, io::Read};

use lexer::{token::{ProcessType, TokenProcess}, Lexer, NewToken};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    SingelineComment,
    Identifier,
    Number,
}

pub fn is_alphanumerical(string:&str) -> bool {
    for ch in string.chars() {
        if !ch.is_alphanumeric() {
            return false
        }
    }
    true
}

fn main() {
    let mut file = File::open("./src/test.txt").unwrap();
    
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    let mut lexer:Lexer<TokenType> = Lexer::new(
        string.chars().collect(), 
        vec![
            NewToken!(TokenType::SingelineComment; '/', 2),
            NewToken!(TokenType::Identifier; ProcessType::String),
            NewToken!(TokenType::Number; x; {
                x.is_alphanumeric()
            }),
        ]
    );

    let tokens = lexer.action();

    println!("tokens: {:?}", tokens);
}