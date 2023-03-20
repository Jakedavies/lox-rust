use crate::{tokens::{Token, TokenType}, parser::Literal, interpreter::RuntimeError, environment::Environment};
use super::expressions::Expression;


#[derive(Debug)]
pub struct UnaryExpression {
    op: Token,
    child: Box<dyn Expression>,
}

impl UnaryExpression {
    pub fn new(op: Token, child: Box<dyn Expression>) -> Self {
        Self { op, child }
    }
}

impl Expression for UnaryExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<Literal, RuntimeError> {
        let child = self.child.evaluate(env)?;
        match &self.op.token_type {
            TokenType::Minus => {
                if let Literal::Number(n) = child {
                    return Ok(Literal::Number(-n));
                }

                panic!("Expected number, got: {:?}", child);
            }
            TokenType::Bang => {
                return match child {
                    Literal::Boolean(b) => Ok(Literal::Boolean(!b)),
                    Literal::Number(n) => Ok(Literal::Boolean(n == 0.0)),
                    Literal::String(s) => Ok(Literal::Boolean(s.len() == 0)),
                };
            }
            _ => {
                panic!("Unexpected token: {:?}", self.op);
            }
        }
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![&self.child]
    }
}
