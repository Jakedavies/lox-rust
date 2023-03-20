use crate::{parser::Literal, interpreter::RuntimeError, environment::Environment};
use super::expressions::Expression;


#[derive(Debug)]
pub struct GroupingExpression {
    child: Box<dyn Expression>,
}

impl GroupingExpression {
    pub fn new(child: Box<dyn Expression>) -> Self {
        Self { child }
    }
}

impl Expression for GroupingExpression {
    fn evaluate(&self, env: &mut Environment) -> Result<Literal, RuntimeError> {
        self.child.evaluate(env)
    }

    fn children(&self) -> Vec<&Box<dyn Expression>> {
        vec![&self.child]
    }
}
