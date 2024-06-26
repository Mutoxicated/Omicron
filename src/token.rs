use std::{fmt::Display, hash::Hasher};

use super::Lexer;

#[macro_export]
macro_rules! CustomTokenEnum {
    ($name:ident; $( $t:tt ),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $( $t ),*
        }
    };
}

pub trait TokenEnum: Display + Clone + PartialEq + Eq + Hasher {
    fn out(lexer:&mut Lexer<Self>) -> bool
    where Self: Sized;
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
    content: String
}

impl<T: TokenEnum> Token<T> {
    pub fn new(r#type: TokenType<T>, content: &str) -> Self {
        Self {
            r#type,
            content: content.to_owned()
        }
    }

    pub fn r#type(&self) -> TokenType<T> {
        self.r#type.clone()
    }

    pub fn content(&self) -> &str {
        self.content.as_str()
    }
}