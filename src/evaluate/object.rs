use std::fmt::{Display, Formatter, Write};
use crate::ast::expression::Identifier;
use crate::ast::statement::BlockStatement;
use crate::evaluate::build_in::BuildInFunction;
use crate::evaluate::environment::Environment;


pub trait Evaluate {
    fn eval(&mut self, environment: &mut Environment) -> anyhow::Result<Object>;
}

#[derive(Debug)]
pub enum Object {
    Int(isize),
    String(String),
    Bool(bool),
    Null,
    Return(Box<Object>),
    Function{
        parameters: Vec<Identifier>,
        body: BlockStatement,
        env: Environment,
    },
    Error(Box<Object>),
    BuildIn(Box<dyn BuildInFunction>),
    Array(Vec<Box<Object>>)
}

impl Object {

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
            },
            Object::String(val) => Object::String(val.clone()),
            Object::BuildIn(val) => Object::BuildIn(val.clone_as_build_in_function()),
            Object::Error(val) => Object::Error(val.clone()),
            Object::Array(val) => Object::Array(val.clone()),
        }
    }
}

impl Object {
    pub fn variant_is_equal(a: &Object, b: &Object) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    pub fn value(&self) -> String {
        match self {
            Object::Int(val) => format!("{}", val),
            Object::Bool(val) => format!("{}", val),
            Object::Null => "null".to_string(),
            Object::Return(val) => format!("{}", val),
            Object::Function { .. } => "fn".to_string(),
            Object::String(val) => format!("{}",val),
            Object::BuildIn(_) => "build_in".to_string(),
            Object::Error(val) => format!("{}", val),
            Object::Array(val) => format!("{:?}", val),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(val) => f.write_str(&format!("{}", val)),
            Object::Bool(val) => f.write_str(&format!("{}", val)),
            Object::Null => f.write_str("null"),
            Object::Return(val) => f.write_str(&format!("ret {}", val)),
            Object::Function { .. } => f.write_str("fn"),
            Object::String(val) => f.write_str(&format!("\"{}\"",val)),
            Object::BuildIn(_) => f.write_str("build_in"),
            Object::Error(val) => f.write_str(&format!("err: {}", val)),
            Object::Array(val) => f.write_str(&format!("{:?}", val))
        }
    }
}