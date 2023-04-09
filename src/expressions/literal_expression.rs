use crate::{tokens::{Token, TokenType}, parser::Literal, interpreter::EvaluationError, environment::Environment};
use super::expressions::{Expression, ExpressionResult};


#[derive(Debug, Clone)]
pub struct LiteralExpression {
    value: Literal,
}

impl LiteralExpression {
    pub fn new(value: Literal) -> Self {
        Self { value }
    }

    pub fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
        Ok(ExpressionResult::Literal(self.value.clone()))
    }

    pub fn children(&self) -> Vec<&Expression> {
        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
