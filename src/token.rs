use std::marker::PhantomData;

use super::Lexer;

pub trait TokenEnum<E: Clone>: Clone + PartialEq + Eq {
    fn out(lexer:&mut Lexer<Self, E>) -> bool 
    where Self: Sized {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token<T: TokenEnum<E>, E: Clone> {
    r#type: T,
    content: String,
    range:(usize, usize),
    line:usize,
    _marker:PhantomData<E>
}

impl<T: TokenEnum<E>, E: Clone> Token<T, E> {
    pub fn new(r#type: T, content: &str, range:(usize, usize), line:usize) -> Self {
        Self {
            r#type,
            content: content.to_owned(),
            range,
            line,
            _marker: PhantomData
        }
    }

    pub fn r#type(&self) -> T {
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