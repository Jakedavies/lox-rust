use std::{rc::Rc, cell::RefCell};

use crate::{environment::Environment, parser::Literal, interpreter::EvaluationError};

use super::expressions::{Expression, ExpressionResult};


#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub name: String,
    child: Box<Expression>,
}

impl AssignmentExpression {
    pub fn new(name: String, child: Box<Expression>) -> Self {
        Self { name, child }
    }

    pub fn evaluate(&self, environment: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
        let v = self.child.evaluate(environment)?;
        environment.set(&self.name, v.clone())?;
        Ok(v)
    }

    pub fn children(&self) -> Vec<&Expression> {
        vec![&self.child]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
