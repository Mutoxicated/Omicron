use std::marker::PhantomData;
use token::{TokenEnum, Token};

pub mod token;

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
    ($access:ident, $( [ $m:tt ] )? _; $($exit:tt)+) => {
        let res = $access.consume();
        if res.is_none() {
            $($exit)+
        }
    };
}

#[macro_export]
macro_rules! lexy {
    ($access:ident; $($exit:tt)+) => {
        peek_check!($access, peek; $($exit)+);

        if peek.is_alphabetic() {
            consume_check!($access, char; $($exit)+);
            $access.push(char);
            conditional_token!($access, is_ascii, [!]is_ascii_control, [!]is_whitespace; $($exit)+);
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

pub struct Lexer<T: TokenEnum<State>, State: Clone> {
    pub state: State,
    buf: Vec<char>,
    index: usize,
    line:usize,
    tokens: Vec<Token<T, State>>,
    buffer:  Vec<char>
}

impl<T: TokenEnum<State>, State: Clone> Lexer<T, State> {
    pub fn with_state(buf: Vec<char>, state: State) -> Self {
        Self {
            state,
            buf,
            index: 0,
            line:0,
            tokens: Vec::new(),
            buffer: Vec::new()
        }
    }

    pub fn action(&mut self) -> Vec<Token<T, State>> {
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
                consume_check!(self, _; break 'a);
                peek_check!(self, char; break 'a);
                peek = char;
            }

            if self.index >= self.buf.len() {
                break
            }

            if T::out(self) {
                self.clear();
                if self.index >= self.buf.len() {
                    break
                }
                continue
            }

            if self.index >= self.buf.len() {
                break
            }
    
            if !self.buffer.is_empty() {
                self.clear();
            }

            consume_check!(self, c; break);
            if c == '\n' {
                self.line += 1;
            }
        }
    }

    pub fn add_token(&mut self, r#type: T, str:&str) {
        self.tokens.push(
            Token::new(r#type, str, (self.index-self.buffer.len(), self.index), self.line)
        );
    }

    pub fn try_lexy(&mut self) -> bool {
        lexy!(self; return false);

        true
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