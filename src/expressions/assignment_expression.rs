use std::{rc::Rc, cell::RefCell};

use crate::{environment::Environment, parser::Literal, interpreter::EvaluationError};

use super::expressions::Expression;


#[derive(Debug)]
pub struct AssignmentExpression {
    pub name: String,
    child: Box<dyn Expression>,
}

impl AssignmentExpression {
    pub fn new(name: String, child: Box<dyn Expression>) -> Self {
        Self { name, child }
    }
}

impl Expression for AssignmentExpression {
    fn evaluate(&self, environment: &mut Environment) -> Result<Literal, EvaluationError> {
        let v = self.child.evaluate(environment)?;
        environment.set(&self.name, v.clone())?;
        Ok(v.clone())
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![&self.child]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
