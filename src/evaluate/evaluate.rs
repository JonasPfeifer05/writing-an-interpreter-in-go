#![allow(unused)]

use crate::ast::statement::Statement;
use crate::evaluate::environment::Environment;
use crate::evaluate::object::Object;
use crate::parser::program::Program;

pub fn eval_program(mut program: Program, environment: &mut Environment) -> anyhow::Result<Object> {
    eval_all(program.statements(), environment, true)
}

pub fn eval(statement: &mut Box<dyn Statement + Send + Sync>, environment: &mut Environment, remove_ret: bool) -> anyhow::Result<Object> {
    let mut result = statement.eval(environment)?;
    if remove_ret {
        match result {
            Object::Return(val) => {
                result = *val;
            }
            _ => {}
        }
    }
    Ok(result)
}

pub fn eval_all(statements: &mut Vec<Box<dyn Statement + Send + Sync>>, environment: &mut Environment, remove_ret: bool) -> anyhow::Result<Object> {
    if statements.is_empty() { return Ok(Object::Null); }

    let mut result = eval(statements.first_mut().unwrap(), environment, false)?;

    match result {
        Object::Return(val) => {
            return if remove_ret {
                Ok(*val)
            } else {
                Ok(Object::Return(val))
            };
        }
        _ => {}
    }

    for statement in statements.iter_mut().skip(1) {
        result = eval(statement, environment, false)?;
        match result {
            Object::Return(val) => {
                return if remove_ret {
                    Ok(*val)
                } else {
                    Ok(Object::Return(val))
                };
            }
            _ => {}
        }
    }

    Ok(result)
}