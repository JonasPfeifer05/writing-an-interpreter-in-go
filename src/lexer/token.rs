use std::fmt::{Debug, Formatter};

/// An enum representing a single token of a program
#[allow(unused)]
#[derive(Eq, PartialEq, Clone)]
pub enum Token {
    // Special Tokens
    Illegal,
    Eof,

    // Identifier and Types
    Ident(String),
    Int(String),

    // Arithmetic Operations
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,

    // Logic Operators
    Bang,
    Equal,
    NotEqual,
    Lt,
    Gt,
    Lte,
    Gte,

    // Separation Characters
    Comma,
    Semicolon,

    // Brackets
    LParent,
    RParent,
    LBrace,
    RBrace,

    // Keywords
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

            Token::Lt => "<",
            Token::Gt => ">",
            Token::Equal => "==",
            Token::NotEqual => "!=",
            Token::Gte => ">=",
            Token::Lte => "<=",

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