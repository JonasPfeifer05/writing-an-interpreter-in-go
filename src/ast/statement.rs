#![allow(unused)]

use std::fmt::{Debug, Display, format, Formatter};
use crate::ast::expression::{Expression, Identifier};
use crate::lexer::token::Token;

pub trait Statement: Display + Debug {}

#[derive(Debug)]
pub struct LetStatement {
    name: Identifier,
    value: Box<dyn Expression>,
}
impl LetStatement {
    pub fn new(name: Identifier, value: Box<dyn Expression>) -> Self {
        Self { name, value }
    }
}
impl Statement for LetStatement {}

impl Display for LetStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("let {} = {};", self.name, self.value))
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    value: Box<dyn Expression>,
}
impl ReturnStatement {
    pub fn new(value: Box<dyn Expression>) -> Self {
        Self { value }
    }
}
impl Statement for ReturnStatement {}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ret {};", self.value))
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    token: Token,
    expression: Box<dyn Expression>,
}
impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<dyn Expression>) -> Self {
        Self { token, expression }
    }
}
impl Statement for ExpressionStatement {}

impl Display for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{};", self.expression))
    }
}

#[derive(Debug)]
pub struct BlockStatement {
    statements: Vec<Box<dyn Statement>>
}
impl BlockStatement {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Self { statements }
    }
}
impl Statement for BlockStatement {}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();

        for statement in &self.statements {
            string.push_str(&format!("{statement}"));
            string.push(' ');
        }

        string.pop();

        f.write_str(&string)
    }
}