use std::fmt::Debug;
use std::io::{BufRead, stdin, stdout, Write};
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

#[derive(Clone, Debug)]
pub struct InputFunction;

impl CloneAsBuildInFunction for InputFunction {
    fn clone_as_build_in_function(&self) -> Box<dyn BuildInFunction> {
        Box::new(self.clone())
    }
}

impl BuildInFunction for InputFunction
{
    fn eval(&mut self, args: Vec<Object>) -> anyhow::Result<Object> {
        if args.len() > 1 { bail!(DifferentAmountOfArguments) }

        let prompt = args.get(0).cloned().unwrap_or(Object::String("".to_string()));

        let prompt = match prompt {
            Object::String(prompt) => prompt,
            _ => bail!(IllegalOperation(Token::Ident("input".to_string()), prompt.clone()))
        };

        let mut buffer = String::new();

        print!("{}", prompt);
        stdout().lock().flush().unwrap();

        stdin().lock().read_line(&mut buffer).unwrap();

        Ok(Object::String(buffer.trim().to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct CastIntFunction;

impl CloneAsBuildInFunction for CastIntFunction {
    fn clone_as_build_in_function(&self) -> Box<dyn BuildInFunction> {
        Box::new(self.clone())
    }
}

impl BuildInFunction for CastIntFunction {
    fn eval(&mut self, args: Vec<Object>) -> anyhow::Result<Object> {
        if args.len() != 1 { bail!(DifferentAmountOfArguments) }

        let string = match &args[0] {
            Object::String(val) => val,
            _ => bail!(IllegalOperation(Token::Ident("input".to_string()), args[0].clone())),
        };

        if let Ok(val) = string.parse::<isize>() {
            Ok(Object::Int(val))
        } else {
            Ok(Object::Error(Box::new(Object::String(format!("Cannot cast {} to int!", string)))))
        }
    }
}

#[derive(Clone, Debug)]
pub struct PrintFunction;

impl CloneAsBuildInFunction for PrintFunction {
    fn clone_as_build_in_function(&self) -> Box<dyn BuildInFunction> {
        Box::new(self.clone())
    }
}

impl BuildInFunction for PrintFunction {
    fn eval(&mut self, args: Vec<Object>) -> anyhow::Result<Object> {
        let mut string = String::new();

        for arg in &args {
            string.push_str(arg.to_string().as_str())
        }

        println!("{}", string);

        Ok(Object::String(string))
    }
}