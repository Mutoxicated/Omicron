use token::{ProcessType, Token, TokenProcess};

pub mod token;

pub struct Lexer<T: Clone> {
    buf: Vec<char>,
    index: usize,
    line:usize,
    tokens: Vec<Token<T>>,
    buffer:  Vec<char>,
    conditionals:Vec<TokenProcess<T>>
}

impl<T: Clone> Lexer<T> {
    pub fn new(buf: Vec<char>, tokens:Vec<TokenProcess<T>>) -> Self {
        Self {
            buf,
            index: 0,
            line:1,
            tokens: Vec::new(),
            buffer: Vec::new(),
            conditionals:tokens
        }
    }

    pub fn action(&mut self) -> Vec<Token<T>> {
        self.get_tokens();

        self.tokens.clone()
    }

    fn get_tokens(&mut self) {
        use ProcessType::*;

        while self.index < self.buf.len() {
            if self.index >= self.buf.len() {
                break
            }

            let c = self.peek().unwrap();
            if c == '\n' {
                self.line += 1;
                self.index += 1;
                continue
            }
            let mut noMatch = true;
            for i in 0..self.conditionals.len() {
                if let CharacterSpecific(x, y) = &self.conditionals[i].process_type {
                    if c == *x {
                        if self.consume_while(*x, *y) {
                            self.add_token(self.conditionals[i].token_type.clone(), &self.read_buffer());
                            self.clear();
                            noMatch = false;
                        }
                    }
                    continue
                }

                if let KeepCollecting(x) = &self.conditionals[i].process_type { 
                    let mut counter = 0;
                    let mut peek = self.peek();
                    while let Some(ch) = peek {
                        if !x(ch) {
                            break
                        }
                        let c = self.peek();
                        self.index += 1;
                        counter += 1;
                        self.buffer.push(c.unwrap());
                        peek = self.peek();
                    }
                    if counter != 0 {
                        self.add_token(self.conditionals[i].token_type.clone(), &self.read_buffer());
                        self.clear();
                        noMatch = false;
                    }
                    continue
                }

                if let String = &self.conditionals[i].process_type {
                    let success = self.consume_while_condition(|c| {
                        c.is_alphabetic()
                    });
                    if success {
                        self.add_token(self.conditionals[i].token_type.clone(), &self.read_buffer());
                        self.clear();
                        noMatch = false;
                    }
                    continue
                }
            }
            if noMatch {
                self.index += 1;
            }
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

    fn consume_while(&mut self, c:char, len:usize) -> bool {
        let mut counter = 0;
        let mut peek = self.peek();
        while let Some(x) = peek {
            if x != c {
                return false
            }
            counter += 1;
            self.push_consume();
            if counter >= len {
                return true
            }
            peek = self.peek();
        }

        self.index -= counter;
        self.clear();
        false
    }

    fn consume(&mut self) -> Option<char> {
        let c = self.peek();
        self.index += 1;
        return c
    }

    pub fn add_token(&mut self, r#type: T, str:&str) {
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

    pub fn push_consume(&mut self) {
        let c = self.consume();
        self.buffer.push(c.unwrap());
    }

    pub fn push_str(&mut self, str:&str) {
        for c in str.chars().collect::<Vec<char>>() {
            self.push(c);
        }
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