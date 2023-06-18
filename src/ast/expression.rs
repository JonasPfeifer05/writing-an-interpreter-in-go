#![allow(unused)]

use std::fmt::{Debug, Display, format, Formatter};
use crate::ast::statement::{BlockStatement, Statement};
use crate::lexer::token::Token;

pub trait Expression: Display + Debug {}

#[derive(Debug)]
pub struct Identifier {
    value: String,
}

impl Identifier {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Expression for Identifier {}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.value))
    }
}

#[derive(Debug)]
pub struct Integer {
    val: String,
}

impl Integer {
    pub fn new(val: String) -> Self {
        Self { val }
    }
}

impl Expression for Integer {}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.val)
    }
}

#[derive(Debug)]
pub struct Boolean {
    val: bool,
}

impl Boolean {
    pub fn new(val: bool) -> Self {
        Self { val }
    }
}

impl Expression for Boolean {}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.val))
    }
}

#[derive(Debug)]
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

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}{})", self.prefix, self.right))
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>,
}

impl InfixExpression {
    pub fn new(left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>) -> Self {
        Self { left, operator, right }
    }
}

impl Expression for InfixExpression {}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({} {} {})", self.left, self.operator, self.right))
    }
}

#[derive(Debug)]
pub struct IfExpression {
    condition: Box<dyn Expression>,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn new(condition: Box<dyn Expression>, consequence: BlockStatement, alternative: Option<BlockStatement>) -> Self {
        Self { condition, consequence, alternative }
    }
}

impl Expression for IfExpression {}

impl Display for IfExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(alternative) = &self.alternative {
            f.write_str(&format!("if ({}) {{ {} }} else {{ {} }}", self.condition, self.consequence, alternative))
        } else {
            f.write_str(&format!("if ({}) {{ {} }}", self.condition, self.consequence))
        }
    }
}

#[derive(Debug)]
pub struct FunctionExpression {
    parameters: Vec<Identifier>,
    body: BlockStatement,
}

impl FunctionExpression {
    pub fn new(parameters: Vec<Identifier>, body: BlockStatement) -> Self {
        Self { parameters, body }
    }
}

impl Expression for FunctionExpression {}

impl Display for FunctionExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut parameters = String::new();
        for param in &self.parameters {
            parameters.push_str(&param.to_string());
            parameters.push(',');
        }
        parameters.pop();
        f.write_str(&format!("fn({}){{ {} }}", parameters, self.body))
    }
}

#[derive(Debug)]
pub struct CallExpression {
    function: Box<dyn Expression>,
    arguments: Vec<Box<dyn Expression>>,
}
impl CallExpression {
    pub fn new(function: Box<dyn Expression>, arguments: Vec<Box<dyn Expression>>) -> Self {
        Self { function, arguments }
    }
}
impl Expression for CallExpression {}
impl Display for CallExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut args = String::new();
        for arg in &self.arguments {
            args.push_str(&arg.to_string());
            args.push(',');
        }
        args.pop();
        f.write_str(&format!("{}({})", self.function, args))
    }
}