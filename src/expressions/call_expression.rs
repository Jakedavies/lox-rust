use crate::{tokens::Token, environment::Environment, parser::Literal, interpreter::EvaluationError};

use super::expressions::{Expression, ExpressionResult};


#[derive(Debug)]
pub struct CallExpression {
    callee: Box<dyn Expression>,
    paren: Token,
    arguments: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(callee: Box<dyn Expression>, paren: Token, arguments: Vec<Box<dyn Expression>>) -> Self {
        Self { callee, paren, arguments }
    }
}

impl Expression for CallExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
        let callee = self.callee.evaluate(env)?;
        let mut args = Vec::new();
        for arg in &self.arguments {
            args.push(arg.evaluate(env)?);
        }

        match callee {
            ExpressionResult::Callable(callable) => {
                let mut new_env = env.enclosed();
                if args.len() != *callable.arity() {
                    return Err(EvaluationError::runtime_error(format!("Expected {} arguments but got {}", callable.arity(), args.len())))
                }
                callable.call(&mut new_env, args)
            }
            _ => Err(EvaluationError::runtime_error(format!("Can only call functions, not {:?}", callee)))
        }
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        return vec![]
    }

    fn as_any(&self) -> &dyn std::any::Any {
        return self
    }
}
