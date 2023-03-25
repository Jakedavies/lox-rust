use crate::{parser::Literal, interpreter::EvaluationError, environment::Environment};
use super::expressions::{Expression, ExpressionResult};

use std::{rc::Rc, cell::RefCell};


#[derive(Debug)]
pub struct GroupingExpression {
    child: Box<dyn Expression>,
}

impl GroupingExpression {
    pub fn new(child: Box<dyn Expression>) -> Self {
        Self { child }
    }
}

impl Expression for GroupingExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<&ExpressionResult, EvaluationError> {
        self.child.evaluate(env)
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![&self.child]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
