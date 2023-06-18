use crate::evaluate::object::Object;
use crate::lexer::token::Token;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Tried to apply illegal operation {0} to {1}!")]
    IllegalOperation(Token, Object)
}