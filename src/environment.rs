use std::collections::HashMap;

use crate::{parser::Literal, interpreter::RuntimeError};



pub struct Environment {
    pub values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn set(&mut self, name: &String, value: Literal) -> Result<(), RuntimeError> {
        match self.values.get_mut(name) {
            Some(val) => {
                *val = value;
                Ok(())
            }
            None => Err(RuntimeError::new(format!("Undefined variable '{}'", name))),
        }
    }

    pub fn get(&self, name: &String) -> Result<&Literal, RuntimeError> {
        match self.values.get(name) {
            Some(value) => Ok(value),
            None => Err(RuntimeError::new(format!("Undefined variable '{}'", name))),
        }
    }
}
