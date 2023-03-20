use crate::{expressions::expressions::Expression, interpreter::RuntimeError, environment::{Environment, self}};


pub trait Statement: std::fmt::Debug {
    fn execute(&self, context: &mut Environment) -> Result<(), RuntimeError>;
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
    fn execute(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        let value = self.expression.evaluate(environment)?;
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
    fn execute(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        self.expression.evaluate(environment)?;
        Ok(())
    }
}


#[derive(Debug)]
pub struct VarStatement {
    name: String,
    initializer: Box<dyn Expression>,
}

impl VarStatement {
    pub fn new(name: String, initializer: Box<dyn Expression>) -> Self {
        Self { name, initializer }
    }
}

impl Statement for VarStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        let value = self.initializer.evaluate(environment)?;
        // TODO. Assign this value to the global environment.
        environment.define(self.name.clone(), value);
        Ok(())
    }
}
