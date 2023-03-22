use std::{any::Any, cell::RefCell, rc::Rc};

use crate::{parser::Literal, interpreter::RuntimeError, environment::{self, Environment}};


pub trait Expression: std::fmt::Debug + Any{
    fn evaluate(&self, env: &mut Environment) -> Result<Literal, RuntimeError>;
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
