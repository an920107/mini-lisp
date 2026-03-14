use std::fmt;

use crate::scanner::token::Token;

#[derive(Debug)]
pub enum ParsingError {
    UnexpectedEnd,
    UnexpectedToken(Token),
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::UnexpectedEnd => write!(f, "unexpected end of input"),
            ParsingError::UnexpectedToken(token) => write!(f, "unexpected token {:?}", token),
        }
    }
}
