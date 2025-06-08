/// T: the token type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token<T> {
    r#type: T,
    content: String,
    range:(usize, usize),
    line:usize,
}

impl<T> Token<T> {
    pub fn new(r#type:T, content: &str, range:(usize, usize), line:usize) -> Self {
        Self { r#type, content: content.to_owned(), range, line }
    }

    pub fn content(&self) -> &str {
        self.content.as_str()
    }

    pub fn change_type(&mut self, new_tt:T) {
        self.r#type = new_tt;
    }
}

pub enum ProcessType {
    KeepCollecting(Box<dyn Fn(char) -> bool>), 
    /// usize: the number of times the character is to be expected
    CharacterSpecific(char, usize),
}

pub struct TokenProcess<T> where Self: Sized {
    pub process_type: ProcessType,
    pub token_type: T
}

impl<T> TokenProcess<T> {
    pub fn new(token_type: T, process_type: ProcessType) -> Self {
        Self{
            process_type,
            token_type
        }
    }
}

/// example: 
/// 
/// NewToken!("//", ProcessType::CharacterSpecific(2))
#[macro_export]
macro_rules! NewToken {
    ($tkt:expr; $t:expr, $size:expr) => {
        TokenProcess::new($tkt, ProcessType::CharacterSpecific($t, $size))
    };
    ($tkt:expr; $id:ident; $t:block) => {
        TokenProcess::new($tkt, ProcessType::KeepCollecting(Box::new(|$id| $t)))
    };
    ($tkt:expr; $id:expr) => {
        TokenProcess::new($tkt, $id)
    };
}