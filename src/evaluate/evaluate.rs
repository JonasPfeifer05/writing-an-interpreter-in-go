use crate::ast::statement::Statement;
use crate::evaluate::object::Object;
use crate::parser::program::Program;

pub fn eval_program(program: Program) -> anyhow::Result<Object> {
    eval_all(program.statements())
}

pub fn eval(statement: &Box<dyn Statement>) -> anyhow::Result<Object> {
    statement.eval()
}

pub fn eval_all(statements: &Vec<Box<dyn Statement>>) -> anyhow::Result<Object> {
    if statements.is_empty() { return Ok(Object::Null) }

    let mut result = eval(statements.first().unwrap())?;

    for statement in statements.iter().skip(1) {
        result = eval(statement)?;
    }

    Ok(result)
}