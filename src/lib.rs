use std::collections::HashMap;

use token::{ProcessType, Token, TokenProcess};

pub mod token;

pub struct Lexer<T: Clone> {
    core: LexerCore<T>,
    data: LexerData<T>
}

struct LexerData<T: Clone> {
    conditionals:Vec<(T, TokenProcess)>
}

impl<T: Clone> LexerData<T> {
    pub fn new(conditionals: Vec<(T, TokenProcess)>) -> Self {
        Self {
            conditionals,
        }
    }
}

struct LexerCore<T: Clone> {
    buf: Vec<char>,
    index: usize,
    line:usize,
    tokens: Vec<Token<T>>,
    buffer:  Vec<char>,
}

impl<T: Clone> LexerCore<T> {
    pub fn new(buf: Vec<char>) -> Self {
        Self {
            buf,
            index: 0,
            line:1,
            tokens: Vec::new(),
            buffer: Vec::new(),
        }
    }

    fn consume_while_condition(&mut self, c:impl Fn(char) -> bool) -> bool {
        let mut counter = 0;
        let mut peek = self.peek();
        while let Some(x) = peek {
            if !c(x) {
                break
            }
            counter += 1;
            self.push_consume();
            peek = self.peek();
        }
        if counter == 0 {
            return false
        }

        true
    }

    fn consume_string(&mut self, str:&str) -> bool {
        if str.is_empty() {
            return false
        }

        let chars:Vec<char> = str.chars().collect();
        let mut counter = 0;
        let mut peek = self.peek();
        while let Some(x) = peek {
            if x != chars[counter] {
                self.index -= counter;
                self.clear();
                return false
            }
            counter += 1;
            self.push_consume();
            peek = self.peek();
            if counter == chars.len() {
                break
            }
        }
   
        true
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.peek();
        self.index += 1;
        return c
    }

    pub fn new_token(&mut self, r#type: T, str:&str) -> Token<T>{
        Token::new(r#type, str, (self.index-self.buffer.len(), self.index), self.line)
    }

    pub fn read_buffer(&self) -> String {
        String::from_iter(&*self.buffer)
    }

    pub fn push_consume(&mut self) {
        let c = self.consume();
        self.buffer.push(c.unwrap());
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn peek(&self) -> Option<char> {
        if self.index > self.buf.len()-1 {
            return None
        }
        Some(self.buf[self.index])
    }

    pub fn make_token(&mut self, data:&LexerData<T>) -> Option<Token<T>> {
        let mut token:Option<Token<T>> = None;
        for x in &data.conditionals {
            if let ProcessType::SpecificString(a) = &x.1.process_type {
                if self.consume_string(a.as_str()) {
                    token = Some(self.new_token(x.0.clone(), &self.read_buffer()));
                    self.clear();
                    break
                }
            }else if let ProcessType::KeepCollecting(a) = &x.1.process_type {
                if self.consume_while_condition(a) {
                    token = Some(self.new_token(x.0.clone(), &self.read_buffer()));
                    self.clear();
                    break
                }
            }
        };

        token
    }
}

impl<T: Clone> Lexer<T> {
    pub fn new(buf: Vec<char>, tokens:Vec<(T, TokenProcess)>) -> Self {
        Self {
            core: LexerCore::new(buf),
            data: LexerData::new(tokens),
        }
    }


    pub fn action(&mut self) -> Vec<Token<T>> {
        self.get_tokens();

        self.core.tokens.clone()
    }

    fn index(&self) -> usize {
        self.core.index
    }

    fn buflen(&self) ->  usize {
        self.core.buf.len()
    }

    fn peek(&self) ->  Option<char> {
        self.core.peek()
    }

    fn get_tokens(&mut self) {
        while self.index() < self.buflen() {
            if self.index() >= self.buflen() {
                break
            }

            let c = self.peek().unwrap();
            if c == '\n' {
                self.core.line += 1;
                self.core.index += 1;
                continue
            }

            let token:Option<Token<T>> = self.core.make_token(&self.data);

            if token.is_none() {
                self.core.index += 1;
                continue
            }
            
            let token = token.unwrap();
            self.core.tokens.push(token);
        }
    }
}