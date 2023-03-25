use std::{any::Any, cell::RefCell, rc::Rc, fmt::{Formatter, Display}};

use crate::{parser::Literal, interpreter::EvaluationError, environment::{self, Environment}, statement::Statement};


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

#[derive(Debug)]
pub enum Callable<'a> {
    Clock,
    UserDefined(&'a Box<dyn Statement>, usize),
}

impl<'a> Clone for Callable<'a> {
    fn clone(&self) -> Self {
        match self {
            Callable::Clock => Callable::Clock,
            Callable::UserDefined(stmt, arity) => {
                Callable::UserDefined(stmt.clone(), *arity)
            },
        }
    }
}


impl<'a> Callable<'a> {
    pub fn arity(&self) -> &usize {
        match self {
            Callable::Clock => &0,
            Callable::UserDefined(stmt, arity) => arity,
        }
    }

    pub fn call(&self, env: &mut Environment, args: Vec<ExpressionResult>) -> Result<ExpressionResult, EvaluationError> {
        match self {
            Callable::Clock => Ok(ExpressionResult::Literal(Literal::Number(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as f64))),
            Callable::UserDefined(stmt, _) => { 
                stmt.execute(env);
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

impl<'a> PartialEq for Callable<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.partial_eq(other)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionResult {
    None,
    Literal(Literal),
    Callable(Callable<'static>),
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

impl ExpressionResult {
    pub fn is_truthy(&self) -> bool {
        match self {
            ExpressionResult::None => false,
            ExpressionResult::Literal(literal) => literal.is_truthy(),
            ExpressionResult::Callable(callable) => true,
        }
    }
}



pub trait Expression: std::fmt::Debug + Any{
    fn evaluate(&self, env: &mut Environment) -> Result<ExpressionResult, EvaluationError>;
    fn children(&self) -> Vec<&Box<dyn Expression>>;
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
