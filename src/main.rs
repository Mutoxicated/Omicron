use std::{fs::File, io::Read};

use lexer::{token::{ProcessType, TokenProcess}, Lexer, NewToken};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    SingelineComment,
    Identifier,
    Number,
    Key
}

macro_rules! strhashmap {
    ($( $e:expr => $e2:expr ),+) => {
        [
            $( ($e.to_owned(), $e2) ),*
        ].iter().cloned().collect()
    };
}

fn main() {
    let mut file = File::open("./src/test.txt").unwrap();
    
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    let mut lexer:Lexer<TokenType> = Lexer::new(
        string.chars().collect(), 
        vec![
            NewToken!(TokenType::SingelineComment; '/', 2),
            NewToken!(TokenType::Identifier; x; {
                x.is_alphabetic() || x == '_' || x == '-'
            }),
            NewToken!(TokenType::Number; x; {
                x.is_alphanumeric()
            }),
        ]
    );
    lexer.with_keywords(strhashmap!["key" => TokenType::Key]);

    let tokens = lexer.action();

    println!("tokens: {:?}", tokens);
}