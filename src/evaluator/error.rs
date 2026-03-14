use crate::{
    evaluator::value::{Value, ValueType},
    scanner::token::Token,
};

pub enum EvaluatingError {
    UndefinedVariable(String),
    TypeError(Token, ValueType, Value),
}

impl std::fmt::Display for EvaluatingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluatingError::UndefinedVariable(name) => write!(f, "undefined variable {}", name),
            EvaluatingError::TypeError(token, expected, real) => {
                write!(
                    f,
                    "type {:?} is expected for {:?}, but got {:?}",
                    expected, token, real
                )
            }
        }
    }
}
