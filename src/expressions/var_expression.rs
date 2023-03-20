use crate::{environment::Environment, parser::Literal, interpreter::RuntimeError};

use super::expressions::Expression;


#[derive(Debug)]
pub struct VarExpression {
    name: String,
}

impl VarExpression {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Expression for VarExpression {
    fn evaluate(&self, environment: &mut Environment) -> Result<Literal, RuntimeError> {
        environment.get(&self.name).cloned()
    }
    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![]
    }
}
