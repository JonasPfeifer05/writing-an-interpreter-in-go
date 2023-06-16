#![allow(unused)]

use std::cell::OnceCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::process::id;
use std::str::Chars;
use crate::lexer::token::Token;
use regex::Regex;

const SKIPS: [char; 4] = [' ', '\n', '\t', '\r'];

pub struct Lexer {
    chars: Vec<char>,
    pointer_position: usize,
    identifier_regex: Regex,
    integer_regex: Regex,
    keywords: HashMap<String, Token>
}

impl Lexer {
    pub fn new<T: ToString>(program: T) -> Self {
        let program = program.to_string();

        let mut keywords = HashMap::new();
        keywords.insert("fn".to_string(), Token::Function);
        keywords.insert("let".to_string(), Token::Let);
        keywords.insert("true".to_string(), Token::True);
        keywords.insert("false".to_string(), Token::False);
        keywords.insert("if".to_string(), Token::If);
        keywords.insert("else".to_string(), Token::Else);
        keywords.insert("ret".to_string(), Token::Return);

        Self {
            chars: program.chars().collect(),
            pointer_position: 0,
            identifier_regex: Regex::new(r"[a-zA-Z_]").unwrap(),
            integer_regex: Regex::new(r"[0-9]").unwrap(),
            keywords,
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

            '=' => {
                match self.peek_char() {
                    Some('=') => {
                        self.move_pointer();
                        Token::Equal
                    }
                    _ => Token::Assign,
                }
            },
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,

            '!' => {
                match self.peek_char() {
                    Some('=') => {
                        self.move_pointer();
                        Token::NotEqual
                    }
                    _ => Token::Bang,
                }
            },

            '<' => {
                match self.peek_char() {
                    Some('=') => {
                        self.move_pointer();
                        Token::LTE
                    },
                    _ => Token::LT,
                }
            },
            '>' => {
                match self.peek_char() {
                    Some('=') => {
                        self.move_pointer();
                        Token::GTE
                    },
                    _ => Token::GT,
                }
            },

            ',' => Token::Comma,
            ';' => Token::Semicolon,

            c if self.identifier_regex.is_match(c.to_string().as_str()) => {
                let ident = self.read_identifier();
                let keyword = self.keywords.get(&ident);
                if let Some(token) = keyword  { token.clone() }
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

    pub fn peek_char(&self) -> Option<&char> {
        self.chars.get(self.pointer_position+1)
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
