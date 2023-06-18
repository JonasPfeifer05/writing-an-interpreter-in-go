use std::fmt::{Display, Formatter};

pub trait Evaluate {
    fn eval(&self) -> anyhow::Result<Object>;
}

#[derive(Debug)]
#[repr(usize)]
pub enum Object {
    Int(isize) = 0,
    Bool(bool) = 1,
    Null = 2,
    Return(Box<Object>) = 3,
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
            Object::Return(val) => f.write_str(&format!("{val}")),
        }
    }
}