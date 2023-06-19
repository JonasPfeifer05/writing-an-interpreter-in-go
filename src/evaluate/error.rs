use crate::evaluate::object::Object;
use crate::lexer::token::Token;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Tried to apply illegal operation {0} to {1}!")]
    IllegalOperation(Token, Object),
    #[error("Tried to apply operation {0} between not matching values {1} and {2}!")]
    MixedTypeOperation(Token, Object, Object),
    #[error("Expected {0}!")]
    UnexpectedObject(String),
    #[error("Found unknown identifier {0}!")]
    UnknownIdentifier(String)
}