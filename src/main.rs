use std::{fs::File, io::Read};

use lexer::{lexy, peek_check, conditional_token, consume_check, token::{TokenEnum, TokenType}, Lexer};

lexer::custom_token_enum!(
    CustomTokenType;
    MacroStart,
    MacroEnd
);

impl TokenEnum for CustomTokenType {
    fn special(lexer:&mut Lexer<Self>) -> bool 
        where Self: Sized {

        use TokenType::*;
        
        let str = lexer.peek_str(2);
        if str.is_none() {
            return false
        }
        let str = str.unwrap();
        if str == "//" {
            let string = lexer.consume_str(2);
            if string.is_none() {
                return false
            }
            let string = string.unwrap();
            lexer.push_str(string.as_str());
            lexer.try_lexy();
            let buf = lexer.read_buffer();
            if buf == "//MACRO_START" {
                lexer.add_token(Custom(CustomTokenType::MacroStart), buf.as_str());
                return true
            }else if buf == "//MACRO_END" {
                lexer.add_token(Custom(CustomTokenType::MacroEnd), buf.as_str());
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

    let mut lexer:Lexer<CustomTokenType> = Lexer::new(string.chars().collect());

    let tokens = lexer.action();

    println!("tokens: {:?}", tokens);
}