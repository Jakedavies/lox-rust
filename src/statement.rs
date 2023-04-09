use crate::{
    environment::{self, Environment},
    expressions::expressions::{Expression, ExpressionResult, Callable},
    interpreter::{ErrorType, EvaluationError},
    parser::Literal,
};

pub trait Executable {
    fn execute(&self, context: &mut Environment) -> Result<(), EvaluationError>;
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(ExpressionStatement),
    Print(PrintStatement),
    Var(VarStatement),
    Block(BlockStatement),
    If(IfStatement),
    While(WhileStatement),
    Function(FunctionStatement),
    Break(BreakStatement),
}

impl Statement {
    pub fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        match self {
            Statement::Expression(statement) => statement.execute(environment),
            Statement::Print(statement) => statement.execute(environment),
            Statement::Var(statement) => statement.execute(environment),
            Statement::Block(statement) => statement.execute(environment),
            Statement::If(statement) => statement.execute(environment),
            Statement::While(statement) => statement.execute(environment),
            Statement::Function(statement) => statement.execute(environment),
            Statement::Break(statement) => statement.execute(environment),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrintStatement {
    expression: Box<Expression>,
}

impl PrintStatement {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

impl Executable for PrintStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        let value = self.expression.evaluate(environment)?;
        println!("{}", value);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    expression: Box<Expression>,
}

impl ExpressionStatement {
    pub fn new(expression: Box<Expression>) -> Self {
        Self { expression }
    }
}

impl Executable for ExpressionStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        self.expression.evaluate(environment)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct VarStatement {
    name: String,
    initializer: Box<Expression>,
}

impl VarStatement {
    pub fn new(name: String, initializer: Box<Expression>) -> Self {
        Self { name, initializer }
    }
}

impl Executable for VarStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        let value = self.initializer.evaluate(environment)?;
        // TODO. Assign this value to the global environment.
        environment.define(self.name.clone(), (value).clone());
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    statements: Vec<Box<Statement>>,
}

impl BlockStatement {
    pub fn new(statements: Vec<Box<Statement>>) -> Self {
        Self { statements }
    }
}

impl Executable for BlockStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        let new_env = &mut environment.enclosed();
        for statement in &self.statements {
            statement.execute(new_env)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    condition: Box<Expression>,
    then_branch: Box<Statement>,
    else_branch: Option<Box<Statement>>,
}

impl IfStatement {
    pub fn new(
        condition: Box<Expression>,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }
}

impl Executable for IfStatement {
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

#[derive(Debug, Clone)]
pub struct WhileStatement {
    condition: Box<Expression>,
    body: Box<Statement>,
}

impl WhileStatement {
    pub fn new(condition: Box<Expression>, body: Box<Statement>) -> Self {
        Self { condition, body }
    }
}

impl Executable for WhileStatement {
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

#[derive(Debug, Clone)]
pub struct FunctionStatement {
    name: String,
    params: Vec<String>,
    body: Box<Statement>,
}

impl FunctionStatement {
    pub fn new(name: String, params: Vec<String>, body: Box<Statement>) -> Self {
        Self { name, params, body }
    }
}

impl Executable for FunctionStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), EvaluationError> {
        let function = Callable::UserDefined(self.body.clone(), self.params.clone());
        environment.define(self.name.clone(), ExpressionResult::Callable(function));
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct BreakStatement {}

impl BreakStatement {
    pub fn new() -> Self {
        Self {}
    }
}

impl Executable for BreakStatement {
    fn execute(&self, _environment: &mut Environment) -> Result<(), EvaluationError> {
        return Err(EvaluationError::break_error());
    }
}
