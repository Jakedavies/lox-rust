use std::{rc::{self, Rc}, cell::RefCell};

use crate::{environment::Environment, parser::Literal, interpreter::EvaluationError};

use super::expressions::{Expression, ExpressionResult};


#[derive(Debug, Clone)]
pub struct VarExpression {
    pub name: String,
}


impl VarExpression {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    pub fn evaluate(&self, environment: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
        environment.get(&self.name)
    }
    pub fn children(&self) -> Vec<&Expression> {
        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
