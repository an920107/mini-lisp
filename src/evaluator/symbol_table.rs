use std::collections::HashMap;

use crate::evaluator::value::Value;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    parent: Option<Box<SymbolTable>>,
    symbols: HashMap<String, Value>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            parent: None,
            symbols: HashMap::new(),
        }
    }

    pub fn with_parent(parent: SymbolTable) -> Self {
        Self {
            parent: Some(Box::new(parent)),
            symbols: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, value: Value) {
        self.symbols.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        if let Some(value) = self.symbols.get(name) {
            Some(value)
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }
}
