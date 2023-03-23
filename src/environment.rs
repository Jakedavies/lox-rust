use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{interpreter::EvaluationError, parser::Literal};

pub struct EnvironmentNode {
    pub values: HashMap<String, Literal>,
    pub parent: Option<Rc<RefCell<EnvironmentNode>>>,
}


impl EnvironmentNode {
    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn set(&mut self, name: &String, value: Literal) -> Result<(), EvaluationError> {
        match (self.values.get_mut(name), self.parent.clone()) {
            (None, None) => Err(EvaluationError::runtime_error(format!("Undefined variable '{}'", name))),
            (None, Some(parent)) => parent.borrow_mut().set(name, value),
            (Some(val), _) => {
                *val = value;
                Ok(())
            }
        }
    }

    pub fn get(&self, name: &String) -> Result<Literal, EvaluationError> {
        match (self.values.get(name), self.parent.clone()) {
            (None, None) => Err(EvaluationError::runtime_error(format!("Undefined variable '{}'", name))),
            (Some(value), _) => Ok((*value).clone()),
            (_, Some(parent)) => {
                let p = parent.borrow();
                let v = p.get(name)?;
                return Ok(v.to_owned().clone());
            }
        }
    }
}

pub struct Environment {
    e: Rc<RefCell<EnvironmentNode>>,
}

impl Clone for Environment {
    fn clone(&self) -> Self {
        Environment {
            e: Rc::clone(&self.e)
        }
    }
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            e: Rc::new(RefCell::new(EnvironmentNode {
                parent: None,
                values: HashMap::new(),
            })),
        }
    }

    pub fn define (&mut self, name: String, value: Literal) {
        self.e.borrow_mut().define(name, value);
    }

    pub fn set(&mut self, name: &String, value: Literal) -> Result<(), EvaluationError> {
        self.e.borrow_mut().set(name, value)
    }

    pub fn get(&self, name: &String) -> Result<Literal, EvaluationError> {
        self.e.borrow().get(name)
    }

    pub fn enclosed(&mut self) -> Self {
        let n = Self::new();
        n.e.borrow_mut().parent = Some(self.e.clone());
        return n;
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
