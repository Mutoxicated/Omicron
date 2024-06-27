use std::{fs::File, io::Read};

use lexer::{token::TokenEnum, Lexer};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    MacroStart,
    MacroEnd
}

impl TokenEnum<String> for TokenType {
    fn out(lexer:&mut Lexer<Self, String>) -> bool 
        where Self: Sized {

        let comment_length = lexer.state.len();
        let str = lexer.peek_str(comment_length);
        if str.is_none() {
            return false
        }
        let str = str.unwrap();

        let comment = lexer.state.clone();
        if str == comment {
            let string = lexer.consume_str(comment_length);
            if string.is_none() {
                return false
            }
            let string = string.unwrap();
            lexer.push_str(string.as_str());
            lexer.try_lexy();
            let buf = lexer.read_buffer();
            if buf == comment.clone()+"MACRO_START" {
                lexer.add_token(TokenType::MacroStart, buf.as_str());
                return true
            }else if buf == comment+"MACRO_END" {
                lexer.add_token(TokenType::MacroEnd, buf.as_str());
                return true
            }
        }

        false
    }
}

fn main() {
    let mut file = File::open("./src/test.txt").unwrap();
    
    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    let mut lexer:Lexer<TokenType, String> = Lexer::with_state(string.chars().collect(), "//".to_owned());

    let tokens = lexer.action();

    println!("tokens: {:?}", tokens);
}