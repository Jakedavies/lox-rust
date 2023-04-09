use crate::{parser::Literal, interpreter::EvaluationError, environment::Environment};
use super::expressions::{Expression, ExpressionResult};

#[derive(Debug, Clone)]
pub struct GroupingExpression {
    child: Box<Expression>,
}

impl GroupingExpression {
    pub fn new(child: Box<Expression>) -> Self {
        Self { child }
    }
    pub fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
        self.child.evaluate(env)
    }

    pub fn children(&self) -> Vec<&Expression> {
        vec![&self.child]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
