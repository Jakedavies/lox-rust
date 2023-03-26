use crate::{
    environment::{self, Environment},
    expressions::expressions::{Expression, ExpressionResult, Callable},
    interpreter::{ErrorType, EvaluationError},
    parser::Literal,
};

pub trait Statement: std::fmt::Debug {
    fn execute(&self, context: &mut Environment) -> Result<(), EvaluationError>;
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
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
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
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
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
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        let value = self.initializer.evaluate(environment)?;
        // TODO. Assign this value to the global environment.
        environment.define(self.name.clone(), (value).clone());
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
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
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
    pub fn new(
        condition: Box<dyn Expression>,
        then_branch: Box<dyn Statement>,
        else_branch: Option<Box<dyn Statement>>,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }
}

impl Statement for IfStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        let condition = self.condition.evaluate(environment)?;
        if condition.is_truthy() {
            self.then_branch.execute(environment)?;
        } else if let Some(else_branch) = &self.else_branch {
            else_branch.execute(environment)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct WhileStatement {
    condition: Box<dyn Expression>,
    body: Box<dyn Statement>,
}

impl WhileStatement {
    pub fn new(condition: Box<dyn Expression>, body: Box<dyn Statement>) -> Self {
        Self { condition, body }
    }
}

impl Statement for WhileStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        while self.condition.evaluate(environment)?.is_truthy() {
            let r = self.body.execute(environment);
            if let Err(e) = r {
                if e.kind == ErrorType::BreakError {
                    break;
                } else {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct FunctionStatement {
    name: String,
    params: Vec<String>,
    body: Box<dyn Statement>,
}

impl FunctionStatement {
    pub fn new(name: String, params: Vec<String>, body: Box<dyn Statement>) -> Self {
        Self { name, params, body }
    }
}

impl Statement for FunctionStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        let function = Callable::UserDefined(&self.body, self.params.len());
        environment.define(self.name.clone(), ExpressionResult::Callable(function));
        Ok(())
    }
}

#[derive(Debug)]
pub struct BreakStatement {}

impl BreakStatement {
    pub fn new() -> Self {
        Self {}
    }
}

impl Statement for BreakStatement {
    fn execute(&self, _environment: &mut Environment) -> Result<(), EvaluationError> {
        return Err(EvaluationError::break_error());
    }
}
