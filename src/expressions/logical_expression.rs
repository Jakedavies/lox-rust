use crate::{tokens::TokenType, environment::Environment, parser::Literal, interpreter::EvaluationError};

use super::expressions::{Expression, ExpressionResult};

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
    fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
        let left = self.left.evaluate(env)?;
        match self.operator {
            LogicalExpressionOperator::And => {
                if !left.is_truthy() {
                    return Ok(ExpressionResult::Literal(Literal::Boolean(false)));
                }
            }
            LogicalExpressionOperator::Or => {
                if left.is_truthy(){
                    return Ok(ExpressionResult::Literal(Literal::Boolean(true)));
                }
            }
        }
        let right = self.right.evaluate(env)?;
        Ok(ExpressionResult::Literal(Literal::Boolean(right.is_truthy())))
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![&self.left, &self.right]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
