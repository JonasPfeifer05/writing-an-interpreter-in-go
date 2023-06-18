#![allow(unused)]

use crate::lexer::token::Token;

#[repr(u8)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Precedences {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Token {
    pub fn precedence(&self) -> Precedences {
        match self {
            Token::Plus |
            Token::Minus => Precedences::Sum,

            Token::Asterisk |
            Token::Slash => Precedences::Product,

            Token::Equal |
            Token::NotEqual => Precedences::Equals,

            Token::Lt |
            Token::Gt |
            Token::Lte |
            Token::Gte => Precedences::LessGreater,

            Token::Bang => Precedences::Prefix,

            _ => Precedences::Lowest
        }
    }
}