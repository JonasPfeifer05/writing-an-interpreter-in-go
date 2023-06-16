#![allow(unused_imports)]

use std::fs;
use regex::Regex;
use writing_an_interpreter_in_go::{
    lexer,
    parser,
    ast
};
use writing_an_interpreter_in_go::lexer::lexer::Lexer;
use writing_an_interpreter_in_go::lexer::token::Token;

fn test_string() -> String {
    fs::read_to_string("res/tests/lexer.txt").expect("Unexpected error while reading file for test!")
}

#[test]
fn test_move_pointer() {
    let program = test_string();
    let mut lexer = Lexer::new(program.clone());

    assert_eq!(lexer.pointer_position(), &0);

    lexer.move_pointer();

    assert_eq!(lexer.pointer_position(), &1);

    lexer.move_pointer();
    lexer.move_pointer();

    assert_eq!(lexer.pointer_position(), &3);

    for _ in 0..program.len() {
        lexer.move_pointer();
    }

    assert!(lexer.move_pointer().is_none())
}

#[test]
fn test_show_char() {
    let program = test_string();
    let mut lexer = Lexer::new(program.clone());

    assert_eq!(lexer.show_char(), Some(&'('));

    lexer.move_pointer();

    assert_eq!(lexer.show_char(), Some(&')'));

    lexer.move_pointer();
    lexer.move_pointer();

    assert_eq!(lexer.show_char(), Some(&'{'));

    for _ in 0..program.len() {
        lexer.move_pointer();
    }

    assert!(lexer.show_char().is_none())
}

#[test]
fn test_next_token() {
    let program = test_string();
    let mut lexer = Lexer::new(program);

    let real_tokens = vec![
        Token::LParent,
        Token::RParent,
        Token::LBrace,
        Token::RBrace,
        Token::Assign,
        Token::Ident("abc".to_string()),
        Token::Int("123".to_string()),
        Token::Plus,
        Token::Let,
        Token::Int("12".to_string()),
        Token::Function,
    ];

    let mut generated_tokens = vec![];
    loop {
        let token = lexer.next_token();
        if token.is_none() { break; }
        generated_tokens.push(token.unwrap());
    }

    assert_eq!(real_tokens.len(), generated_tokens.len());

    for i in 0..real_tokens.len() {
        assert_eq!(real_tokens[i], generated_tokens[i])
    }
}

#[test]
fn test_generate_tokens() {
    let program = test_string();
    let mut lexer = Lexer::new(program);

    let real_tokens = vec![
        Token::LParent,
        Token::RParent,
        Token::LBrace,
        Token::RBrace,
        Token::Assign,
        Token::Ident("abc".to_string()),
        Token::Int("123".to_string()),
        Token::Plus,
        Token::Let,
        Token::Int("12".to_string()),
        Token::Function,
        Token::Eof,
    ];

    assert_eq!(real_tokens, lexer.generate_tokens())
}