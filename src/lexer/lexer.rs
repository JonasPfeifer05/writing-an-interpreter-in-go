#![allow(unused)]

use std::cell::OnceCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::process::id;
use std::str::Chars;
use crate::lexer::token::Token;
use regex::Regex;

/// Every character that is not important for the interpreter
const SKIPS: [char; 4] = [' ', '\n', '\t', '\r'];

/// A struct for converting a string into the separate tokens
pub struct Lexer {
    /// The chars of the program parsed
    chars: Vec<char>,
    /// The current position inside the program
    pointer_position: usize,
    /// Regex to identify identifiers
    identifier_regex: Regex,
    /// Regex to identify integers
    integer_regex: Regex,
    /// A map of every keyword
    keywords: HashMap<String, Token>
}

impl Lexer {
    /// Create a new lexer ready to split the passed program into token
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
        keywords.insert("err".to_string(), Token::Error);
        keywords.insert("while".to_string(), Token::While);

        Self {
            chars: program.chars().collect(),
            pointer_position: 0,
            identifier_regex: Regex::new(r"[a-zA-Z_]").unwrap(),
            integer_regex: Regex::new(r"[0-9]").unwrap(),
            keywords,
        }
    }

    /// Converts the passed program into a list of tokens
    pub fn generate_tokens(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        while let Some(token) = self.next_token() { tokens.push(token) }
        tokens.push(Token::Eof);
        tokens
    }

    /// Checks if there are any chars left to read
    pub fn out_of_chars(&self) -> bool {
        self.pointer_position >= self.chars.len()
    }

    /// Returns the next token from the current pointer position and also moves the pointer
    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespaces();

        if self.out_of_chars() { return None; }

        let token = match self.current_char().unwrap() {
            '(' => Token::LParent,
            ')' => Token::RParent,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,

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
            '%' => Token::Modular,

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
                        Token::Lte
                    },
                    _ => Token::Lt,
                }
            },
            '>' => {
                match self.peek_char() {
                    Some('=') => {
                        self.move_pointer();
                        Token::Gte
                    },
                    _ => Token::Gt,
                }
            },
            '|' => {
                match self.peek_char() {
                    Some('|') => {
                        self.move_pointer();
                        Token::Or
                    }
                    _ => Token::Illegal,
                }
            }
            '&' => {
                match self.peek_char() {
                    Some('&') => {
                        self.move_pointer();
                        Token::And
                    }
                    _ => Token::Illegal,
                }
            }
            ',' => Token::Comma,
            ';' => Token::Semicolon,

            '"' => Token::String(self.read_string()),

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

    fn read_string(&mut self) -> String {
        self.move_pointer();

        let mut buf = String::new();

        while !self.out_of_chars() && self.current_char().unwrap() != &'"' {
           buf.push(self.current_char().unwrap().clone());
            self.move_pointer();
        }

        buf
    }

    /// Read an integer from the program as long as possible
    fn read_integer(&mut self) -> String {
        let mut integer = String::new();

        while !self.out_of_chars() && self.integer_regex.is_match(self.current_char().unwrap().to_string().as_str()) {
            integer.push(*self.current_char().unwrap());
            self.move_pointer();
        }

        self.move_pointer_back();

        integer
    }

    /// Read an identifier from the program as long as possible
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while !self.out_of_chars() && self.identifier_regex.is_match(self.current_char().unwrap().to_string().as_str()) {
            identifier.push(*self.current_char().unwrap());
            self.move_pointer();
        }

        self.move_pointer_back();

        identifier
    }

    /// Skips every non important character (Whitespaces, Newline, etc.) from the current position till the next useful character
    fn skip_whitespaces(&mut self) {
        while !self.out_of_chars() && SKIPS.contains(self.current_char().unwrap()) {
            self.move_pointer();
        }
    }

    /// Returns the char on the current pointer position inside the program
    pub fn current_char(&self) -> Option<&char> {
        self.chars.get(self.pointer_position)
    }

    /// Returns the char one advanced from the current pointer position inside the program
    pub fn peek_char(&self) -> Option<&char> {
        self.chars.get(self.pointer_position+1)
    }

    /// Moves the pointer one index towards the end of the program
    pub fn move_pointer(&mut self) -> Option<()> {
        if self.out_of_chars() { return None; }
        self.pointer_position += 1;
        Some(())
    }

    /// Moves the pointer one index away the end of the program
    pub fn move_pointer_back(&mut self) -> Option<()> {
        if self.pointer_position == 0 { return None; }
        self.pointer_position -= 1;
        Some(())
    }

    /// Get all chars from the passed program
    pub fn chars(&self) -> &Vec<char> {
        &self.chars
    }

    // Get the current pointer position
    pub fn pointer_position(&self) -> &usize {
        &self.pointer_position
    }
}
