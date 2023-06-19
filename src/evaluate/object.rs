use std::fmt::{Display, Formatter};
use crate::ast::expression::Identifier;
use crate::ast::statement::BlockStatement;
use crate::evaluate::environment::Environment;
use crate::evaluate::evaluate::eval;

pub trait Evaluate {
    fn eval(&self, environment: &mut Environment) -> anyhow::Result<Object>;
}

#[derive(Debug)]
pub enum Object {
    Int(isize),
    Bool(bool),
    Null,
    Return(Box<Object>),
    Function{
        parameters: Vec<Identifier>,
        body: BlockStatement,
        env: Environment,
    },
}

impl Clone for Object {
    fn clone(&self) -> Self {
        match self {
            Object::Int(val) => Object::Int(val.clone()),
            Object::Bool(val) => Object::Bool(val.clone()),
            Object::Null => Object::Null,
            Object::Return(val) => Object::Return(val.clone()),
            Object::Function { parameters, body, env} => Object::Function {
                parameters: parameters.clone(),
                body: body.clone_as_block_statement(),
                env: env.clone(),
            }
        }
    }
}

impl Object {
    pub fn variant_is_equal(a: &Object, b: &Object) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(val) => f.write_str(&format!("{val}")),
            Object::Bool(val) => f.write_str(&format!("{val}")),
            Object::Null => f.write_str("null"),
            Object::Return(val) => f.write_str(&format!("ret {val}")),
            _ => f.write_str("fn")
        }
    }
}