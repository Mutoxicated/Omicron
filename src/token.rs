use super::Lexer;

pub trait TokenEnum: Clone + PartialEq + Eq {
    fn lexy(lexer:&mut Lexer<Self>) -> bool
    where Self: Sized;
    fn special(lexer:&mut Lexer<Self>) -> bool 
    where Self: Sized {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType<T> {
    Lexy,
    Number,
    Custom(T)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token<T: TokenEnum> {
    r#type: TokenType<T>,
    content: String,
    range:(usize, usize),
    line:usize,
}

impl<T: TokenEnum> Token<T> {
    pub fn new(r#type: TokenType<T>, content: &str, range:(usize, usize), line:usize) -> Self {
        Self {
            r#type,
            content: content.to_owned(),
            range,
            line
        }
    }

    pub fn r#type(&self) -> TokenType<T> {
        self.r#type.clone()
    }

    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    pub fn index(&self) -> (usize, usize) {
        self.range
    }

    pub fn line(&self) -> usize {
        self.line
    }
}