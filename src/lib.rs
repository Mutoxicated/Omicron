use std::marker::PhantomData;
use token::{TokenEnum, Token, TokenType};

pub mod token;

#[macro_export]
macro_rules! custom_token_enum {
    ($name:ident; $( $t:tt ),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $( $t ),*
        }
    };
}

#[macro_export]
macro_rules! char_token {
    ($access:ident, $ch:expr, $t:expr, $(exit:tt)*) => {
        if $access.buf[$access.index] == $ch {
            let char = $access.consume();
            $access.push(char);
            $access.add_token(
                $t, char.to_string().as_str()
            );
            $access.clear();
            $($exit)*
        }
    };
    ($access:ident, $ch:expr, $t:expr) => {
        if $access.buf[$access.index] == $ch {
            let char = $access.consume();
            $access.push(char);
            $access.add_token(
                $t, char.to_string().as_str()
            );
            $access.clear();
            continue;
        }
    };
}
#[macro_export]
macro_rules! keyword_token {
    ($access:ident, $keyword:expr, $t:expr) => {
        if $access.read_buffer() == $keyword {
            $access.add_token(
                $t, $keyword
            );
        }
    };
    ($access:ident, $keyword:expr, $t:expr, $($exit:tt)*) => {
        if $access.read_buffer() == $keyword {
            $access.add_token(
                $t, $keyword
            );
            $($exit)*
        }
    };
}

#[macro_export]
macro_rules! peek_check {
    ($access:ident, $name:tt; $t:tt) => {
        let res = $access.peek();
        if res.is_none() {
            $t
        }
        let $name = res.unwrap();
    };
}
#[macro_export]
macro_rules! consume_check {
    ($access:ident, $( [ $m:tt ] )? $name:ident; $t:tt) => {
        let res = $access.peek();
        if res.is_none() {
            $t
        }
        let $($m)? $name = res.unwrap();
    };
}

pub struct Lexer<T: TokenEnum> {
    buf: Vec<char>,
    index: usize,
    line:usize,
    tokens: Vec<Token<T>>,
    buffer:  Vec<char>,
    _marker: PhantomData<T>
}

impl<T: TokenEnum> Lexer<T> {
    pub fn new(buf: Vec<char>) -> Self {
        Self {
            buf,
            index: 0,
            line:0,
            tokens: Vec::new(),
            buffer: Vec::new(),
            _marker: PhantomData
        }
    }

    pub fn action(&mut self) -> Vec<Token<T>> {
        self.get_tokens();

        self.tokens.clone()
    }

    fn get_tokens(&mut self) {
 
        macro_rules! conditional_token {
            ($( $( [$t:tt] )? $cond:ident ),*) => {
                peek_check!(self, peek; break);
                while $( $($t)? peek.$cond() ) && * {
                    consume_check!(self, char; break);
                    self.buffer.push(char);
                    if self.index >= self.buf.len() {
                        break
                    }
                }
            };
        }

        

        while self.index < self.buf.len() {
            if self.index >= self.buf.len() {
                break
            }

            consume_check!(self, [mut] consumed; break);

            while consumed == ' ' {
                let res = self.consume();
                if res.is_none() {
                    break
                }
                consumed = res.unwrap();
                if self.index >= self.buf.len() {
                    break
                }
            }

            if self.index >= self.buf.len() {
                break
            }

            if T::special(self) {
                self.clear();
                if self.index >= self.buf.len() {
                    break
                }
                continue
            }

            peek_check!(self, peek; break);
    
            if peek.is_alphabetic() {
                consume_check!(self, char; break);
                self.push(char);
                conditional_token!(is_ascii, [!]is_ascii_control);
            }

            // custom tokens
            if T::lexy(self) {
                self.clear();
                continue
            }

            if !self.buffer.is_empty() {
                let string = String::from_iter(&*self.buffer);
                let token = Token::new(TokenType::Lexy, string.as_str(), (self.index-self.buffer.len(), self.index), self.line);
                self.clear();
                self.tokens.push(token);
                if self.index >= self.buf.len() {
                    break
                }
            }

            peek_check!(self, peek; break);
    
            if peek.is_alphanumeric() {
                consume_check!(self, char; break);
                self.push(char);
                conditional_token!(is_alphanumeric);
                let string = self.read_buffer();
                let token = Token::new(TokenType::Number, string.as_str(), (self.index-self.buffer.len(), self.index), self.line);
                self.clear();
                self.tokens.push(token);
                continue
            }
            
            consume_check!(self, c; break);
            if c == '\n' {
                self.line += 1;
            }
        }
    }

    pub fn add_token(&mut self, r#type: TokenType<T>, str:&str) {
        self.tokens.push(
            Token::new(r#type, str, (self.index-self.buffer.len(), self.index), self.line)
        );
    }

    pub fn read_buffer(&self) -> String {
        String::from_iter(&*self.buffer)
    }

    pub fn push(&mut self, c:char) {
        self.buffer.push(c);
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn consume(&mut self) -> Option<char> {
        if self.index > self.buf.len()-1 {
            return None
        }
        let char = self.buf[self.index];
        self.index += 1;
        Some(char)
    }

    pub fn peek(&self) -> Option<char> {
        if self.index > self.buf.len()-1 {
            return None
        }
        Some(self.buf[self.index])
    }

    pub fn peek_off(&self, offset:usize) -> Option<char> {
        if self.index+offset > self.buf.len()-1 {
            return None
        }
        Some(self.buf[self.index+offset])
    }

    pub fn peek_str(&mut self, str:&str) -> bool {
        let mut index = 0;
        for c in str.chars().collect::<Vec<char>>() {
            let res = self.peek_off(index);
            if res.is_none() {
                return false
            }
            let peek = res.unwrap();
            if peek != c {
                return false
            }else {
                self.push(c);
                index += 1;
            }
        }
        
        true
    }
}