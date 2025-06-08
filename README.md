# Omicron

Omicron is a simple and flexible lexer library written in rust and designed to be easily and quickly used. Part of its flexibility comes from the fact that the user can implement their own token type enum and link to each token a specific process that gives the lexer the conditions to create the token. 

**The result:** the user can focus only on the token types and how they are processed, without having to actually write a whole lexer for it!

## Setup

```rust
// create the lexer
let mut lexer:Lexer<TokenType> = Lexer::new(
    // the file string to tokenize from
    file_contents,
    // the token processes
    vec![
        tokenProc!(TokenType::SinglelineComment; '/', 2),
        tokenProc!(TokenType::Identifier; x; {
            x.is_alphabetic() || x == '_' || x == '-'
        }),
        tokenProc!(TokenType::Number; x; {
            x.is_alphanumeric()
        }),
    ]
);
// optional keyword processing
lexer.with_keywords(keywords!["key" => TokenType::Key, "word" => TokenType::Word]);

let tokens = lexer.action() // boom!
```

## Info

### Token
This is what a token consists of in Omicron:
```rust
/// T: the token type implemented by the end user
pub struct Token<T> {
    r#type: T,
    content: String,
    /// (column_end, column_start)
    range:(usize, usize),
    line:usize,
}
```

### TokenProcess
You create a token process with the tokenProc! macro. Though in reality this creates a tuple of type (T, TokenProcess), thus linking the specified process with the given token type.

### Keywords
Keywords match on the processed tokens to check if the contents match theirs. It is therefore assumed that, for keywords to work, you need to have some basic token processes like an identifier token.

###More info
The project is documented with rust documentation comments, so whatever else you want, it's most likely either at the lib.rs or tokens.rs file!
