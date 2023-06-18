#![allow(unused)]

use std::fmt::Debug;
use crate::ast::expression::{Expression, Identifier};
use crate::lexer::token::Token;

pub trait Statement: Debug {}


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