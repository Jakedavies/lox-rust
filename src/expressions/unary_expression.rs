use crate::{tokens::{Token, TokenType}, parser::Literal, interpreter::EvaluationError, environment::Environment, expressions::expressions::ExpressionResult};
use super::expressions::Expression;


#[derive(Debug, Clone)]
pub struct UnaryExpression {
    op: Token,
    child: Box<Expression>,
}

impl UnaryExpression {
    pub fn new(op: Token, child: Box<Expression>) -> Self {
        Self { op, child }
    }

    pub fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
        let child = self.child.evaluate(env)?;
        match &self.op.token_type {
            TokenType::Minus => {
                if let ExpressionResult::Literal(Literal::Number(n)) = child {
                    return Ok(ExpressionResult::Literal(Literal::Number(-n)));
                }

                panic!("Expected number, got: {:?}", child);
            }
            TokenType::Bang => {
                return match child {
                    ExpressionResult::Literal(Literal::Boolean(b)) => Ok(ExpressionResult::Literal(Literal::Boolean(!b))),
                    ExpressionResult::Literal(Literal::Number(n)) => Ok(ExpressionResult::Literal(Literal::Boolean(n == 0.0))),
                    ExpressionResult::Literal(Literal::String(s)) => Ok(ExpressionResult::Literal(Literal::Boolean(s.len() == 0))),
                    _ => Err(EvaluationError::runtime_error("Expected boolean, number or string".to_string()))
                };
            }
            _ => {
                panic!("Unexpected token: {:?}", self.op);
            }
        }
    }

    pub fn children(&self) -> Vec<&Expression> {
        vec![&self.child]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
