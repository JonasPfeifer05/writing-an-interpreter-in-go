use crate::ast::statement::Statement;
use crate::evaluate::environment::Environment;
use crate::evaluate::object::Object;
use crate::parser::program::Program;

pub fn eval_program(program: Program, environment: &mut Environment) -> anyhow::Result<Object> {
    eval_all(program.statements(), environment)
}

pub fn eval(statement: &Box<dyn Statement + Send + Sync>,environment: &mut Environment) -> anyhow::Result<Object> {
    statement.eval(environment)
}

pub fn eval_all(statements: &Vec<Box<dyn Statement + Send + Sync>>, environment: &mut Environment) -> anyhow::Result<Object> {
    if statements.is_empty() { return Ok(Object::Null) }

    let mut result = eval(statements.first().unwrap(),environment)?;

    for statement in statements.iter().skip(1) {
        result = eval(statement, environment)?;
        match result {
            Object::Return(val) => {
                result = *val;
                break
            },
            _ => {}
        }
    }

    Ok(result)
}