use crate::lexer::token::Token;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parser didnt expect token: {0}")]
    UnexpectedToken(Token),
    #[error("Ran out of tokens!")]
    RanOutOfTokens,
}