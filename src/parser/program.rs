#![allow(unused)]

use crate::ast::statement::Statement;

#[derive(Debug, Default)]
pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Program {
    pub fn add_statement(&mut self, statement: Box<dyn Statement>) {
        self.statements.push(statement);
    }


    pub fn statements(&self) -> &Vec<Box<dyn Statement>> {
        &self.statements
    }
}