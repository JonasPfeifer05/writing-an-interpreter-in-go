#![allow(unused)]

use crate::ast::precedences::Precedences::Prefix;
use crate::lexer::token::Token;

#[repr(u8)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum Precedences {
    Lowest,
    OrAnd,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Assign,
}

impl Token {
    pub fn precedence(&self) -> Precedences {
        match self {
            Token::Plus |
            Token::Minus => Precedences::Sum,
            Token::Or |
            Token::And => Precedences::OrAnd,

            Token::Asterisk |
            Token::Slash |
            Token::Modular => Precedences::Product,

            Token::Equal |
            Token::NotEqual => Precedences::Equals,

            Token::Lt |
            Token::Gt |
            Token::Lte |
            Token::Gte => Precedences::LessGreater,

            Token::Bang => Precedences::Prefix,

            Token::LParent => Precedences::Call,

            Token::Assign => Precedences::Assign,

            _ => Precedences::Lowest
        }
    }
}