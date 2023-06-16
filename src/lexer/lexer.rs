#![allow(unused)]

use std::cell::OnceCell;
use std::ops::Deref;
use std::process::id;
use std::str::Chars;
use crate::lexer::token::Token;
use regex::Regex;

const SKIPS: [char; 3] = [' ', '\n', '\t'];

pub struct Lexer {
    chars: Vec<char>,
    pointer_position: usize,
    identifier_regex: Regex,
    integer_regex: Regex,
}

impl Lexer {
    pub fn new<T: ToString>(program: T) -> Self {
        let program = program.to_string();
        Self {
            chars: program.chars().collect(),
            pointer_position: 0,
            identifier_regex: Regex::new(r"[a-zA-Z_]").unwrap(),
            integer_regex: Regex::new(r"[0-9]").unwrap(),
        }
    }

    pub fn generate_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        loop {
            let token = self.next_token();
            if token.is_none() { break }
            tokens.push(token.unwrap());
        }
        tokens.push(Token::Eof);
        tokens
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        if self.pointer_position == self.chars.len() { return None; }

        let token = match self.show_char().unwrap() {
            '(' => Token::LParent,
            ')' => Token::RParent,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '=' => Token::Assign,
            '+' => Token::Plus,
            c if self.identifier_regex.is_match(c.to_string().as_str()) => {
                let ident = self.read_identifier();
                if &ident == "let" { Token::Let }
                else if &ident == "function" { Token::Function }
                else { Token::Ident(ident) }
            },
            c if self.integer_regex.is_match(c.to_string().as_str()) => Token::Int(self.read_integer()),
            _ => Token::Illegal,
        };

        self.move_pointer();

        Some(token)
    }

    fn read_integer(&mut self) -> String {
        let mut integer = String::new();

        while self.show_char().is_some() && self.integer_regex.is_match(self.show_char().unwrap().to_string().as_str()) {
            integer.push(*self.show_char().unwrap());
            self.move_pointer();
        }

        self.move_pointer_back();

        integer
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while self.show_char().is_some() && self.identifier_regex.is_match(self.show_char().unwrap().to_string().as_str()) {
            identifier.push(*self.show_char().unwrap());
            self.move_pointer();
        }

        self.move_pointer_back();

        identifier
    }

    fn skip_whitespaces(&mut self) {
        while self.show_char().is_some() && SKIPS.contains(self.show_char().unwrap()) {
            self.move_pointer();
        }
    }

    pub fn show_char(&self) -> Option<&char> {
        self.chars.get(self.pointer_position)
    }

    pub fn move_pointer(&mut self) -> Option<()> {
        if self.pointer_position == self.chars.len() { return None; }
        self.pointer_position += 1;
        Some(())
    }

    pub fn move_pointer_back(&mut self) -> Option<()> {
        if self.pointer_position == 0 { return None; }
        self.pointer_position -= 1;
        Some(())
    }


    pub fn chars(&self) -> &Vec<char> {
        &self.chars
    }
    pub fn pointer_position(&self) -> &usize {
        &self.pointer_position
    }
}
