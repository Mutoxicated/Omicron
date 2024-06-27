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
    ($access:ident, $ch:expr, $t:expr; $($exit:tt)*) => {
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
}
#[macro_export]
macro_rules! keyword_token {
    ($access:ident, $keyword:expr, $t:expr; $($exit:tt)*) => {
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
    ($access:ident, $([ $m:tt ])? $name:ident; $($exit:tt)+) => {
        let res = $access.peek();
        if res.is_none() {
            $($exit)+
        }
        let $($m)? $name = res.unwrap();
    };
}
#[macro_export]
macro_rules! consume_check {
    ($access:ident, $( [ $m:tt ] )? $name:ident; $($exit:tt)+) => {
        let res = $access.consume();
        if res.is_none() {
            $($exit)+
        }
        let $($m)? $name = res.unwrap();
    };
}

#[macro_export]
macro_rules! lexy {
    ($access:ident; $($exit:tt)+) => {
        peek_check!($access, peek; $($exit)+);

        if peek.is_alphabetic() {
            consume_check!($access, char; $($exit)+);
            $access.push(char);
            conditional_token!($access, is_ascii, [!]is_ascii_control; $($exit)+);
        }
    };
}

#[macro_export]
macro_rules! conditional_token {
    ($access:ident, $( $( [$t:tt] )? $cond:ident),*; $($exit:tt)+) => {
        peek_check!($access, [mut] char; $($exit)+);
        while $( $($t)? char.$cond() ) && * {
            consume_check!($access, c; $($exit)+);
            $access.push(c);
            peek_check!($access, c; $($exit)+);
            char = c;
        }
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

        'a: while self.index < self.buf.len() {
            if self.index >= self.buf.len() {
                break
            }

            peek_check!(self, [mut] peek; break);

            while peek == ' ' {
                consume_check!(self, consumed; break 'a);
                peek_check!(self, char; break 'a);
                peek = char;
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

            if self.index >= self.buf.len() {
                break
            }
    
            lexy!(self; break);

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
                conditional_token!(self, is_alphanumeric; break 'a);
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

    pub fn push_str(&mut self, str:&str) {
        for c in str.chars().collect::<Vec<char>>() {
            self.push(c);
        }
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

    pub fn consume_str(&mut self, len:usize) -> Option<String> {
        let mut string = String::new();
        for _ in 0..len {
            consume_check!(self, char; return None);
            string.push(char);
        }

        Some(string)
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

    pub fn peek_str(&self, len:usize) -> Option<String> {
        let mut string = String::new();
        for i in 0..len {
            let char = self.peek_off(i);
            char?;
            let char = char.unwrap();
            string.push(char);
        }

        Some(string)
    }
}