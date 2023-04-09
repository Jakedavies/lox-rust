use crate::{tokens::Token, environment::Environment, parser::Literal, interpreter::EvaluationError};

use super::expressions::{Expression, ExpressionResult};


#[derive(Debug, Clone)]
pub struct CallExpression {
    callee: Box<Expression>,
    paren: Token,
    arguments: Vec<Box<Expression>>,
}

impl CallExpression {
    pub fn new(callee: Box<Expression>, paren: Token, arguments: Vec<Box<Expression>>) -> Self {
        Self { callee, paren, arguments }
    }

    pub fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
        let callee = self.callee.evaluate(env)?;
        let mut args = Vec::new();
        let mut new_env = env.enclosed();
        match callee {
            ExpressionResult::Callable(callable) => {
                if self.arguments.len() != callable.arity() {
                    return Err(EvaluationError::runtime_error(format!("Expected {} arguments but got {}", callable.arity(), args.len())))
                }
                for arg in &self.arguments {
                    args.push(arg.evaluate(env)?);
                }

                callable.call(&mut new_env, args)
            }
            _ => Err(EvaluationError::runtime_error(format!("Can only call functions, not {:?}", callee)))
        }
    }

    pub fn children(&self) -> Vec<&Expression> {
        return vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        return self
    }
}
