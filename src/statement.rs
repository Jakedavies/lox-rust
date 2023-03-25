use crate::{
    environment::{self, Environment},
    expressions::expressions::{Expression, ExpressionResult},
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
        environment.define(self.name.clone(), (*value).clone());
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

#[derive(Debug)]
pub enum Callable {
    Clock,
    UserDefined(Box<dyn Statement>, usize),
}

impl Callable {
    fn arity(&self) -> &usize {
        match self {
            Callable::Clock => &0,
            Callable::UserDefined(stmt, arity) => arity,
        }
    }

    fn call(
        &self,
        env: &mut Environment,
        args: Vec<ExpressionResult>,
    ) -> Result<(), EvaluationError> {
        match self {
            Callable::Clock => {
                let now = std::time::SystemTime::now();
                let since_the_epoch = now
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards");
                let in_ms = since_the_epoch.as_millis();
                let value = Literal::Number(in_ms as f64);
                print!("{}", value);
                Ok(())
            }
            Callable::UserDefined(stmt, _) => stmt.execute(env),
        }
    }

    fn partial_eq(&self, other: &Callable) -> bool {
        match self {
            Callable::Clock => match other {
                Callable::Clock => true,
                _ => false,
            },
            Callable::UserDefined(stmt, arity) => false,
        }
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        self.partial_eq(other)
    }
}
