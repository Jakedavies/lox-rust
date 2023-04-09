use crate::{tokens::TokenType, environment::Environment, parser::Literal, interpreter::EvaluationError};

use super::expressions::{Expression, ExpressionResult};

#[derive(Debug, Clone)]
pub enum LogicalExpressionOperator {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct LogicalExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operator: LogicalExpressionOperator,
}

impl LogicalExpression {
    pub fn new(left:Box<Expression>, right: Box<Expression>, operator: LogicalExpressionOperator) -> Self {
        Self { left, right, operator }
    }
    pub fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
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

    pub fn children(&self) -> Vec<&Expression> {
        vec![&self.left, &self.right]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
