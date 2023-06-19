use std::fmt::Debug;
use anyhow::bail;
use crate::evaluate::error::EvalError::{DifferentAmountOfArguments, IllegalOperation};
use crate::evaluate::object::Object;
use crate::lexer::token::Token;

pub trait BuildInFunction: CloneAsBuildInFunction + Debug + Send + Sync {
    fn eval(&mut self, args: Vec<Object>) -> anyhow::Result<Object>;
}

pub trait CloneAsBuildInFunction {
    fn clone_as_build_in_function(&self) -> Box<dyn BuildInFunction>;
}

#[derive(Clone, Debug)]
pub struct LenFunction;

impl CloneAsBuildInFunction for LenFunction {
    fn clone_as_build_in_function(&self) -> Box<dyn BuildInFunction> {
        Box::new(self.clone())
    }
}

impl BuildInFunction for LenFunction {
    fn eval(&mut self, mut args: Vec<Object>) -> anyhow::Result<Object> {
        if args.len() != 1 { bail!(DifferentAmountOfArguments) }
        let obj = args.pop().unwrap();
        Ok(match obj {
            Object::String(val) => Object::Int(val.len() as isize),
            _ => bail!(IllegalOperation(Token::Ident("len".to_string()), obj))
        })
    }
}