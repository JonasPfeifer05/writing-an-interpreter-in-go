#![allow(unused)]

use std::fmt::{Debug, Display, format, Formatter};
use crate::ast::expression::{Expression, Identifier};
use crate::evaluate::environment::Environment;
use crate::evaluate::evaluate::eval_all;
use crate::evaluate::object::{Evaluate, Object};
use crate::evaluate::object::Object::Return;
use crate::lexer::token::Token;

pub trait CloneAsStatement: Send {
    fn clone_as_statement(&self) -> Box<dyn Statement + Send + Sync>;
}

pub trait Statement: Display + Debug + Evaluate + CloneAsStatement + Send + Sync {}

#[derive(Debug)]
pub struct LetStatement {
    name: Identifier,
    value: Box<dyn Expression + Send + Sync>,
}
impl LetStatement {
    pub fn new(name: Identifier, value: Box<dyn Expression + Send + Sync>) -> Self {
        Self { name, value }
    }
}

impl Evaluate for LetStatement {
    fn eval(&self, environment: &mut Environment) -> anyhow::Result<Object> {
        let val = self.value.eval(environment)?;
        environment.set(self.name.to_string(), val.clone());
        Ok(val)
    }
}

impl CloneAsStatement for LetStatement {
    fn clone_as_statement(&self) -> Box<dyn Statement + Send + Sync> {
        Box::new(LetStatement::new(self.name.clone(), self.value.clone_as_expression()))
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
    value: Box<dyn Expression + Send + Sync>,
}
impl ReturnStatement {
    pub fn new(value: Box<dyn Expression + Send + Sync>) -> Self {
        Self { value }
    }
}

impl Evaluate for ReturnStatement {
    fn eval(&self, environment: &mut Environment) -> anyhow::Result<Object> {
        Ok(Object::Return(Box::new(self.value.eval(environment)?)))
    }
}

impl CloneAsStatement for ReturnStatement {
    fn clone_as_statement(&self) -> Box<dyn Statement + Send + Sync> {
        Box::new(ReturnStatement::new(self.value.clone_as_expression()))
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
    expression: Box<dyn Expression + Send + Sync>,
}
impl ExpressionStatement {
    pub fn new(token: Token, expression: Box<dyn Expression + Send + Sync>) -> Self {
        Self { token, expression }
    }
}

impl Evaluate for ExpressionStatement {
    fn eval(&self, environment: &mut Environment) -> anyhow::Result<Object> {
        self.expression.eval(environment)
    }
}

impl CloneAsStatement for ExpressionStatement {
    fn clone_as_statement(&self) -> Box<dyn Statement + Send + Sync> {
        Box::new(ExpressionStatement::new(self.token.clone(), self.expression.clone_as_expression()))
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
    statements: Vec<Box<dyn Statement + Send + Sync>>
}
impl BlockStatement {
    pub fn new(statements: Vec<Box<dyn Statement + Send + Sync>>) -> Self {
        Self { statements }
    }


    pub fn statements(&self) -> &Vec<Box<dyn Statement + Send + Sync>> {
        &self.statements
    }

    pub fn clone_as_block_statement(&self) -> BlockStatement {
        let statements: Vec<_> = self.statements.iter().map(|x| x.clone_as_statement()).collect();
        BlockStatement::new(statements)
    }
}

impl Evaluate for BlockStatement {
    fn eval(&self, environment: &mut Environment) -> anyhow::Result<Object> {
        eval_all(&self.statements, environment)
    }
}

impl CloneAsStatement for BlockStatement {
    fn clone_as_statement(&self) -> Box<dyn Statement + Send + Sync> {
        let statements: Vec<_> = self.statements.iter().map(|x| x.clone_as_statement()).collect();
        Box::new(BlockStatement::new(statements))
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