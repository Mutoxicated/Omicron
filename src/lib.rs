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
            $access.add_token(
                $t, char.to_string().as_str(), $access.index(), $access.line()
            );
            $($exit)*
        }
    };
    ($access:ident, $ch:expr, $t:expr) => {
        if $access.buf[$access.index] == $ch {
            let char = $access.consume();
            $access.add_token(
                $t, char.to_string().as_str(), $access.index(), $access.line()
            );
            continue;
        }
    };
}
#[macro_export]
macro_rules! keyword_token {
    ($access:ident, $keyword:expr, $t:expr) => {
        if $access.read_buffer() == $keyword {
            $access.add_token(
                $t, $keyword, $access.index(), $access.line()
            );
        }
    };
    ($access:ident, $keyword:expr, $t:expr, $($exit:tt)*) => {
        if $access.read_buffer() == $keyword {
            $access.add_token(
                $t, $keyword, $access.index(), $access.line()
            );
            $($exit)*
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
 
        macro_rules! conditional_token {
            ($cond:ident) => {
                while self.buf[self.index].$cond() {
                    let char = self.consume();
                    self.buffer.push(char);
                    if self.index >= self.buf.len() {
                        break
                    }
                }
            };
        }

        while self.index < self.buf.len() {
            while self.buf[self.index] == ' ' {
                self.consume();
                if self.index >= self.buf.len() {
                break
            }
            }

            if self.index >= self.buf.len() {
                break
            }
    
            if self.buf[self.index].is_alphabetic() {
                let char = self.consume();
                self.buffer.push(char);
                conditional_token!(is_alphabetic);
            }

            // custom tokens
            if T::out(self) {
                self.buffer.clear();
                continue
            }

            if !self.buffer.is_empty() {
                let string = String::from_iter(&*self.buffer);
                let token = Token::new(TokenType::Lexy, string.as_str(), self.index, self.line);
                self.buffer.clear();
                self.tokens.push(token);
            }
    
            if self.buf[self.index].is_alphanumeric() {
                let char = self.consume();
                self.buffer.push(char);
                conditional_token!(is_alphanumeric);
                let string = self.read_buffer();
                let token = Token::new(TokenType::Number, string.as_str(), self.index, self.line);
                self.buffer.clear();
                self.tokens.push(token);
                continue
            }
            
            let c = self.consume();
            if c == '\n' {
                self.line += 1;
            }
        }
    }

    pub fn add_token(&mut self, r#type: TokenType<T>, str:&str) {
        self.tokens.push(
            Token::new(r#type, str, self.index, self.line)
        );
    }

    pub fn read_buffer(&self) -> String {
        String::from_iter(&*self.buffer)
    }

    fn consume(&mut self) -> char {
        let char = self.buf[self.index];
        self.index += 1;
        char
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn line(&self) -> usize {
        self.line
    }
}