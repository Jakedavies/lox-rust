use crate::{tokens::{Token, TokenType}, parser::Literal, interpreter::EvaluationError, environment::Environment};
use super::expressions::{Expression, ExpressionResult};


#[derive(Debug)]
pub struct LiteralExpression {
    value: Literal,
}

impl LiteralExpression {
    pub fn new(value: Literal) -> Self {
        Self { value }
    }
}

impl Expression for LiteralExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
        Ok(ExpressionResult::Literal(self.value.clone()))
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
