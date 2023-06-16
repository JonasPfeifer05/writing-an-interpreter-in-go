use std::fmt::{Debug, Formatter};

#[allow(unused)]
#[derive(Eq, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(String),

    Assign,
    Plus,

    Comma,
    Semicolon,

    LParent,
    RParent,

    LBrace,
    RBrace,

    Function,
    Let,
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let representation = match self {
            Token::Illegal => "Illegal",
            Token::Eof => "EOF",

            Token::Ident(name) => name.as_str(),
            Token::Int(val) => val.as_str(),

            Token::Assign => "=",
            Token::Plus => "+",

            Token::Comma => ",",
            Token::Semicolon => ";",

            Token::LParent => "(",
            Token::RParent => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",

            Token::Function => "function",
            Token::Let => "let",
        };
        f.write_str(representation)
    }
}