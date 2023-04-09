use std::{any::Any, cell::RefCell, rc::Rc, fmt::{Display, Formatter}};

use crate::{parser::Literal, interpreter::EvaluationError, environment::{self, Environment}, statement::{Executable, Statement}, tokens::Token};

use super::{binary_expression::BinaryExpression, grouping_expression::GroupingExpression, unary_expression::UnaryExpression, literal_expression::LiteralExpression, call_expression::CallExpression, logical_expression::LogicalExpression, var_expression::VarExpression, assignment_expression::AssignmentExpression};


/**
pub trait Callable: std::fmt::Debug + Any {
    fn arity(&self) -> usize;
    fn call(&self, env: &mut Environment, args: Vec<ExpressionResult>) -> Result<ExpressionResult, EvaluationError>;
}


#[derive(Debug, Clone, PartialEq)]
pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, env: &mut Environment, args: Vec<ExpressionResult>) -> Result<ExpressionResult, EvaluationError> {
        Ok(ExpressionResult::Literal(Literal::Number(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as f64)))
    }
}
*/

#[derive(Debug, Clone)]
pub enum Callable {
    Clock,
    UserDefined(Box<Statement>, Vec<String>),
}



impl Callable {
    pub fn arity(&self) -> usize {
        match self {
            Callable::Clock => 0,
            Callable::UserDefined(_stmt, params) => params.len(),
        }
    }

    pub fn call(&self, env: &mut Environment, args: Vec<ExpressionResult>) -> Result<ExpressionResult, EvaluationError> {
        
        match self {
            Callable::Clock => Ok(ExpressionResult::Literal(Literal::Number(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as f64))),
            Callable::UserDefined(stmt, arg_names) => { 
                for (i, arg) in args.iter().enumerate() {
                    env.define(arg_names[i].clone(), arg.clone());
                }

                let result = stmt.execute(env);
                if let Err(e) = result {
                    return Err(e);
                }
                Ok(ExpressionResult::None)
            },
        }
    }

    fn partial_eq(&self, other: &Callable) -> bool {
        match self {
            Callable::Clock => match other {
                Callable::Clock => true,
                _ => false,
            },
            Callable::UserDefined(stmt, arity) => false
        }
    }
}

impl<'a> PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        self.partial_eq(other)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionResult {
    None,
    Literal(Literal),
    Callable(Callable),
}

impl<'a> ExpressionResult{
    pub fn is_truthy(&self) -> bool {
        match self {
            ExpressionResult::None => false,
            ExpressionResult::Literal(literal) => literal.is_truthy(),
            ExpressionResult::Callable(callable) => true,
        }
    }
}

impl Display for ExpressionResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionResult::None => write!(f, "None"),
            ExpressionResult::Literal(literal) => write!(f, "{}", literal),
            ExpressionResult::Callable(callable) => write!(f, "Callable"),
        }
    }
}


/**
pub trait Expression: std::fmt::Debug + Any{
    fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError>;
    fn children(&self) -> Vec<&Expression>;
    fn as_any(&self) -> &dyn Any;
}

impl dyn Expression {
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Any,
    {
        self.as_any().downcast_ref()
    }
}
*/

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
    Unary(UnaryExpression),
    Call(CallExpression),
    Literal(LiteralExpression),
    Logical(LogicalExpression),
    Var(VarExpression),
    Assignment(AssignmentExpression),
}

impl Expression {
    pub fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError> {
        match self {
            Expression::Binary(expr) => expr.evaluate(env),
            Expression::Grouping(expr) => expr.evaluate(env),
            Expression::Unary(expr) => expr.evaluate(env),
            Expression::Call(expr) => expr.evaluate(env),
            Expression::Literal(expr) => expr.evaluate(env),
            Expression::Logical(expr) => expr.evaluate(env),
            Expression::Var(expr) => expr.evaluate(env),
            Expression::Assignment(expr) => expr.evaluate(env),
        }
    }

    pub fn children(&self) -> Vec<&Expression> {
        match self {
            Expression::Binary(expr) => expr.children(),
            Expression::Grouping(expr) => expr.children(),
            Expression::Unary(expr) => expr.children(),
            Expression::Call(expr) => expr.children(),
            Expression::Literal(expr) => expr.children(),
            Expression::Logical(expr) => expr.children(),
            Expression::Var(expr) => expr.children(),
            Expression::Assignment(expr) => expr.children(),
        }
    }
}

