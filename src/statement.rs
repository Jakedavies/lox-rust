use crate::{expressions::expressions::Expression, interpreter::RuntimeError};


pub trait Statement: std::fmt::Debug {
    fn execute(&self) -> Result<(), RuntimeError>;
}

#[derive(Debug)]
pub struct PrintStatement {
    expression: Box<dyn Expression>,
}

impl PrintStatement {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
}

impl Statement for PrintStatement {
    fn execute(&self) -> Result<(), RuntimeError> {
        let value = self.expression.evaluate()?;
        println!("{}", value);
        Ok(())
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    expression: Box<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
}

impl Statement for ExpressionStatement {
    fn execute(&self) -> Result<(), RuntimeError> {
        self.expression.evaluate()?;
        Ok(())
    }
}

