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

pub enum ProcessType where Self: Sized {
    /// the field in it is essentially a predicate 
    KeepCollecting(Box<dyn Fn(char) -> bool>), 
    /// usize: the number of times the character is to be expected
    CharacterSpecific(char, usize),
}

pub struct TokenProcess {
    pub process_type: ProcessType,
}

impl TokenProcess {
    pub fn new(process_type: ProcessType) -> Self {
        Self{
            process_type,
        }
    }
}

/// examples: 
/// 
/// ```
/// tokenProc!(TokenType::SinglineComment, '/', ProcessType::CharacterSpecific(2)) 
/// // expands into: (TokenType::SinglineComment, TokenProcess::new(ProcessType::CharacterSpecific('/, 2)))
/// 
/// tokenProc!(TokenType::Identifier, x; { x.isAlphabetical() }) 
/// // expands into: (TokenType::Identifier, TokenProcess::new(ProcessType::KeepCollecting(Box::new(|x| { x.isAlphabetical() }))))
/// 
/// ```
#[macro_export]
macro_rules! tokenProc {
    ($tkt:expr; $t:expr, $size:expr) => {
        ($tkt, TokenProcess::new(ProcessType::CharacterSpecific($t, $size)))
    };
    ($tkt:expr; $id:ident; $t:block) => {
        ($tkt, TokenProcess::new(ProcessType::KeepCollecting(Box::new(|$id| { $t }))))
    };
}

/// example: 
/// 
/// ```
/// keywords!("class" => TokenType::Class, "pub" => TokenType::Pub)
/// ```
#[macro_export]
macro_rules! keywords {
    ($( $e:expr => $e2:expr ),+) => {
        [
            $( ($e.to_owned(), $e2) ),*
        ].iter().cloned().collect()
    };
}