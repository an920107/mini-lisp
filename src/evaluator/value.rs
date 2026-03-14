use crate::{evaluator::symbol_table::SymbolTable, parser::ast::FunctionExpression};

#[derive(Debug, Clone)]
pub struct Closure {
    pub function: FunctionExpression,
    pub env: SymbolTable,
}

#[derive(Debug, Clone)]
pub enum Value {
    Boolean(bool),
    Integer(i128),
    Function(Closure),
}

#[derive(Debug)]
pub enum ValueType {
    Boolean,
    Integer,
    Function,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            _ => false,
        }
    }
}
