use std::{fs::File, io::Read};

use lexer::{token::{ProcessType, TokenProcess}, Lexer, tokenProc, keywords};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    SinglelineComment,
    Identifier,
    Number,
    Key,
    Word
}

fn main() {
    let mut file = File::open("./src/test.txt").unwrap();
    
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    let mut lexer:Lexer<TokenType> = Lexer::new(
        string.chars().collect(), 
        vec![
            tokenProc!(TokenType::SinglelineComment; '/', 2),
            tokenProc!(TokenType::Identifier; x; {
                x.is_alphabetic() || x == '_' || x == '-'
            }),
            tokenProc!(TokenType::Number; x; {
                x.is_alphanumeric()
            }),
        ]
    );
    lexer.with_keywords(keywords!["key" => TokenType::Key, "word" => TokenType::Word]);

    let tokens = lexer.action();

    println!("tokens: {:?}", tokens);
}