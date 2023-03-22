
use crate::{expressions::expressions::Expression, interpreter::RuntimeError, environment::{Environment, self}, parser::Literal};


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

#[derive(Debug)]
pub struct BlockStatement {
    statements: Vec<Box<dyn Statement>>,
}

impl BlockStatement {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Self { statements }
    }
}

impl Statement for BlockStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        let new_env = &mut environment.enclosed();
        for statement in &self.statements {
            statement.execute(new_env)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct IfStatement {
    condition: Box<dyn Expression>,
    then_branch: Box<dyn Statement>,
    else_branch: Option<Box<dyn Statement>>,
}

impl IfStatement {
    pub fn new(condition: Box<dyn Expression>, then_branch: Box<dyn Statement>, else_branch: Option<Box<dyn Statement>>) -> Self {
        Self { condition, then_branch, else_branch }
    }
}

impl Statement for IfStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), RuntimeError> {
        let condition = self.condition.evaluate(environment)?;
        if let Literal::Boolean(b) = condition {
            if b {
                self.then_branch.execute(environment)?;
            } else if let Some(else_branch) = &self.else_branch {
                else_branch.execute(environment)?;
            }
        }
        Ok(())
    }
}
