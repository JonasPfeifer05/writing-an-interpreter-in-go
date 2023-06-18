#![allow(unused)]

use std::fmt::{Debug, Formatter};
use crate::ast::statement::Statement;
use crate::lexer::token::Token;

pub trait Expression: Debug {}

pub struct Identifier {
    value: String,
}
impl Identifier {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
impl Expression for Identifier {}
impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.value))
    }
}

pub struct Integer {
    val: String,
}
impl Integer {
    pub fn new(val: String) -> Self {
        Self { val }
    }
}
impl Expression for Integer {}
impl Debug for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.val)
    }
}

pub struct Boolean {
    val: bool
}
impl Boolean {
    pub fn new(val: bool) -> Self {
        Self { val }
    }
}
impl Expression for Boolean {}
impl Debug for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.val))
    }
}

pub struct PrefixExpression {
    prefix: Token,
    right: Box<dyn Expression>,
}
impl PrefixExpression {
    pub fn new(prefix: Token, right: Box<dyn Expression>) -> Self {
        Self { prefix, right }
    }
}
impl Expression for PrefixExpression {}
impl Debug for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({:?}{:?})", self.prefix, self.right))
    }
}

pub struct InfixExpression {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>
}
impl InfixExpression {
    pub fn new(left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>) -> Self {
        Self { left, operator, right }
    }
}
impl Expression for InfixExpression {}
impl Debug for InfixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({:?} {:?} {:?})", self.left, self.operator, self.right))
    }
}

#[derive(Debug)]
pub struct IfExpression{
    condition: Box<dyn Expression>,
    consequence: Vec<Box<dyn Statement>>,
    alternative: Option<Vec<Box<dyn Statement>>>,
}
impl Statement for IfExpression {}

