use std::fmt::{Debug, Formatter};

#[allow(unused)]
#[derive(Eq, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(String),

    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,

    Bang,
    Equal,
    NotEqual,
    GTE,
    LTE,

    LT,
    GT,

    Comma,
    Semicolon,

    LParent,
    RParent,

    LBrace,
    RBrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
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
            Token::Minus => "-",
            Token::Asterisk => "*",
            Token::Slash => "/",

            Token::Bang => "!",

            Token::LT => "<",
            Token::GT => ">",
            Token::Equal => "==",
            Token::NotEqual => "!=",
            Token::GTE => ">=",
            Token::LTE => "<=",

            Token::Comma => ",",
            Token::Semicolon => ";",

            Token::LParent => "(",
            Token::RParent => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",

            Token::Function => "function",
            Token::Let => "let",
            Token::True => "true",
            Token::False => "false",
            Token::If => "if",
            Token::Else => "else",
            Token::Return => "ret",
        };
        f.write_str(&format!("'{representation}'"))
    }
}