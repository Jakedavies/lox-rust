use std::{collections::HashMap, cell::RefCell, rc::Rc};

use crate::{parser::Literal, interpreter::RuntimeError};



pub struct Environment {
    parent : Option<Rc<RefCell<Environment>>>,
    pub values: HashMap<String, Literal>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }

    pub fn enclosed(mut self) -> Self {
        let mut n = Self::new();
        n.parent = Some(Rc::new(RefCell::new(self)));
        return n
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn set(&mut self, name: &String, value: Literal) -> Result<(), RuntimeError> {
        match (self.values.get_mut(name), self.parent.clone()) {
            (None, None) => Err(RuntimeError::new(format!("Undefined variable '{}'", name))),
            (None, Some(parent)) => parent.borrow_mut().set(name, value),
            (Some(val), _) => {
                *val = value;
                Ok(())
            }
        }
    }

    pub fn get(&self, name: &String) -> Result<Literal, RuntimeError> {
        match (self.values.get(name), self.parent.clone()) {
            (None, None) => Err(RuntimeError::new(format!("Undefined variable '{}'", name))),
            (Some(value), _) => Ok((*value).clone()),
            (_, Some(parent)) => {
                let p = parent.borrow();
                let v = p.get(name)?;
                return Ok(v.to_owned().clone())
            }
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment() {
        let mut env = Environment::new();
        env.define("a".to_string(), Literal::Number(1.0));
        env.define("b".to_string(), Literal::Number(2.0));
        env.define("c".to_string(), Literal::Number(3.0));

        assert_eq!(env.get(&"a".to_string()).unwrap(), Literal::Number(1.0));
        assert_eq!(env.get(&"b".to_string()).unwrap(), Literal::Number(2.0));
        assert_eq!(env.get(&"c".to_string()).unwrap(), Literal::Number(3.0));
    }

    #[test]
    fn nested_environments() {
        let mut env = Environment::new();
        env.define("a".to_string(), Literal::Number(1.0));
        env.define("b".to_string(), Literal::Number(2.0));

        let mut env2 = env.enclosed();
        env2.define("b".to_string(), Literal::Number(6.0));

        assert_eq!(env2.get(&"b".to_string()).unwrap(), Literal::Number(6.0));
        assert_eq!(env2.get(&"a".to_string()).unwrap(), Literal::Number(1.0));
    }

    #[test]
    fn nested_reassignment() {
        let mut env = Environment::new();
        env.define("a".to_string(), Literal::Number(1.0));

        let mut env2 = env.enclosed();

        env2.set(&"a".to_string(), Literal::Number(2.0));
        assert_eq!(env2.get(&"a".to_string()).unwrap(), Literal::Number(2.0));
    }
}
