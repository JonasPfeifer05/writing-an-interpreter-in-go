#![allow(unused)]

use std::fmt::{Display, Formatter};
use crate::ast::statement::Statement;

#[derive(Default, Debug)]
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

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from("program: {\n");
        for statement in &self.statements {
            string.push_str("    ");
            string.push_str(&statement.to_string());
            string.push('\n');
        }
        string.push('}');
        f.write_str(&string)
    }
}