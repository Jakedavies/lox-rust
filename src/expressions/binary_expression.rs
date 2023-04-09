use crate::{tokens::{Token, TokenType}, parser::Literal, interpreter::EvaluationError, environment::Environment};
use super::expressions::{Expression, ExpressionResult};
use std::{rc::Rc, cell::RefCell};


#[derive(Debug, Clone)]
pub struct BinaryExpression {
    op: Token,
    left: Box<Expression>,
    right: Box<Expression>,
}

impl BinaryExpression {
    pub fn new(op: Token, left: Box<Expression>, right: Box<Expression>) -> Self {
        Self { op, left, right }
    }
    pub fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
        let left = self.left.evaluate(env)?;
        let right = self.right.evaluate(env)?;

        match &self.op.token_type {
                        TokenType::Plus => match (&left, &right) {
                (ExpressionResult::Literal(Literal::Number(n)), ExpressionResult::Literal(Literal::Number(m))) => Ok(ExpressionResult::Literal(Literal::Number(n + m))),
                (ExpressionResult::Literal(Literal::String(n)), ExpressionResult::Literal(Literal::String(m))) => Ok(ExpressionResult::Literal(Literal::String(n.to_owned() + m.as_str()))),
                (ExpressionResult::Literal(Literal::String(n)), ExpressionResult::Literal(Literal::Number(m))) => Ok(ExpressionResult::Literal(Literal::String(n.to_owned() + m.to_string().as_str()))),
                _ => Err(EvaluationError::runtime_error(format!("Expected two numbers or two strings, got: {:?} {:?}", left, right)))
            },
            TokenType::Minus => match (&left, &right) {
                (ExpressionResult::Literal(Literal::Number(n)), ExpressionResult::Literal(Literal::Number(m))) => Ok(ExpressionResult::Literal(Literal::Number(n - m))),
                _ => Err(EvaluationError::runtime_error(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::Star => match (&left, &right) {
                (ExpressionResult::Literal(Literal::Number(n)), ExpressionResult::Literal(Literal::Number(m))) => Ok(ExpressionResult::Literal(Literal::Number(n * m))),
                _ => Err(EvaluationError::runtime_error(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::Slash => match (&left, &right) {
                (ExpressionResult::Literal(Literal::Number(n)), ExpressionResult::Literal(Literal::Number(m))) => Ok(ExpressionResult::Literal(Literal::Number(n / m))),
                _ => Err(EvaluationError::runtime_error(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::Greater => match (&left, &right) {
                (ExpressionResult::Literal(Literal::Number(n)), ExpressionResult::Literal(Literal::Number(m))) => Ok(ExpressionResult::Literal(Literal::Boolean(n > m))),
                _ => Err(EvaluationError::runtime_error(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::GreaterEqual => match (&left, &right) {
                (ExpressionResult::Literal(Literal::Number(n)), ExpressionResult::Literal(Literal::Number(m))) => Ok(ExpressionResult::Literal(Literal::Boolean(n >= m))),
                _ => Err(EvaluationError::runtime_error(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::Less => match (&left, &right) {
                (ExpressionResult::Literal(Literal::Number(n)), ExpressionResult::Literal(Literal::Number(m))) => Ok(ExpressionResult::Literal(Literal::Boolean(n < m))),
                _ => Err(EvaluationError::runtime_error(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::LessEqual => match (&left, &right) {
                (ExpressionResult::Literal(Literal::Number(n)), ExpressionResult::Literal(Literal::Number(m))) => Ok(ExpressionResult::Literal(Literal::Boolean(n <= m))),
                _ => Err(EvaluationError::runtime_error(format!("Expected two numbers, got: {:?} {:?}", left, right)))
            },
            TokenType::BangEqual => match (&left, &right) {
                (ExpressionResult::Literal(Literal::Number(n)), ExpressionResult::Literal(Literal::Number(m))) => Ok(ExpressionResult::Literal(Literal::Boolean(n != m))),
                (ExpressionResult::Literal(Literal::String(n)), ExpressionResult::Literal(Literal::String(m))) => Ok(ExpressionResult::Literal(Literal::Boolean(n != m))),
                (ExpressionResult::Literal(Literal::Boolean(n)), ExpressionResult::Literal(Literal::Boolean(m))) => Ok(ExpressionResult::Literal(Literal::Boolean(n != m))),
                _ => Err(EvaluationError::runtime_error(format!("Expected two numbers, two strings, or two booleans, got: {:?} {:?}", left, right)))
            },
            TokenType::EqualEqual => {
                return match (&left, &right) {
                    (ExpressionResult::Literal(Literal::Number(n)), ExpressionResult::Literal(Literal::Number(m))) => Ok(ExpressionResult::Literal(Literal::Boolean(n == m))),
                    (ExpressionResult::Literal(Literal::String(n)), ExpressionResult::Literal(Literal::String(m))) => Ok(ExpressionResult::Literal(Literal::Boolean(n == m))),
                    (ExpressionResult::Literal(Literal::Boolean(n)), ExpressionResult::Literal(Literal::Boolean(m))) => Ok(ExpressionResult::Literal(Literal::Boolean(n == m))),
                    _ => Err(EvaluationError::runtime_error(format!("Expected two numbers, two strings, or two booleans, got: {:?} {:?}", left, right)))
                };
            }
            _ => {
                panic!("Unexpected token: {:?}", self.op);
            }
        }
    }

    pub fn children(&self) -> Vec<&Expression> {
        vec![&self.left, &self.right]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
