use crate::{tokens::{Token, TokenType}, parser::Literal, interpreter::RuntimeError, environment::Environment};
use super::expressions::Expression;

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
    fn evaluate(&self, env: &mut Environment) -> Result<Literal, RuntimeError> {
        Ok(self.value.clone())
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
