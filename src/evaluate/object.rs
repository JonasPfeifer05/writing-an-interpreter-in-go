use std::fmt::{Display, Formatter};

pub trait Evaluate {
    fn eval(&self) -> anyhow::Result<Object>;
}

#[derive(Debug)]
pub enum Object {
    Int(isize),
    Bool(bool),
    Null,
    Return(Box<Object>),
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(val) => f.write_str(&format!("{val}")),
            Object::Bool(val) => f.write_str(&format!("{val}")),
            Object::Null => f.write_str("null"),
            Object::Return(val) => f.write_str(&format!("{val}")),
        }
    }
}