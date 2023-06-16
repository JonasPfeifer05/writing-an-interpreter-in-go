#![allow(unused)]

use std::str::Chars;
use crate::parser::token::Token;

pub struct Lexer {
    chars: Vec<char>,
    pointer_position: usize,
}

impl Lexer {
    pub fn new<T: ToString>(program: T) -> Self {
        let program = program.to_string();
        Self {
            chars: program.chars().collect(),
            pointer_position: 0,
        }
    }

    pub fn generate_tokens(&mut self) -> Vec<Token> {
        todo!();
    }

    pub fn next_token(&mut self) -> Token {
        todo!();
    }

    pub fn char(&self) -> Option<&char> {
        todo!();
    }

    pub fn move_pointer(&mut self) -> Option<()> {
        todo!();
    }
}
