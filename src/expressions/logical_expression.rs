use crate::{tokens::TokenType, environment::Environment, parser::Literal, interpreter::RuntimeError};

use super::expressions::Expression;

#[derive(Debug)]
pub enum LogicalExpressionOperator {
    And,
    Or,
}

#[derive(Debug)]
pub struct LogicalExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    pub operator: LogicalExpressionOperator,
}

impl LogicalExpression {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>, operator: LogicalExpressionOperator) -> Self {
        Self { left, right, operator }
    }
}

impl Expression for LogicalExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<Literal, RuntimeError> {
        let left = self.left.evaluate(env)?;
        match self.operator {
            LogicalExpressionOperator::And => {
                if left == Literal::Boolean(false){
                    return Ok(Literal::Boolean(false));
                }
            }
            LogicalExpressionOperator::Or => {
                if left == Literal::Boolean(true) {
                    return Ok(Literal::Boolean(true));
                }
            }
        }
        let right = self.right.evaluate(env)?;
        Ok(Literal::Boolean(right == Literal::Boolean(true)))
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![&self.left, &self.right]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
