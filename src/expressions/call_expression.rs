use crate::{tokens::Token, environment::Environment, parser::Literal, interpreter::EvaluationError};

use super::expressions::Expression;


#[derive(Debug)]
pub struct CallExpression {
    callee: Box<dyn Expression>,
    paren: Token,
    arguments: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(callee: Box<dyn Expression>, paren: Token, arguments: Vec<Box<dyn Expression>>) -> Self {
        Self { callee, paren, arguments }
    }
}

impl Expression for CallExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<Literal, EvaluationError> {
        return Err(EvaluationError::runtime_error("Not implemented".to_string()))
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        return vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        return self
    }
}
