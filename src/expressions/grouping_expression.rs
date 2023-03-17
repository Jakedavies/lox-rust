use crate::tree::{Node, Evaluatable};


#[derive(Debug)]
pub struct GroupingExpression {
    child: Box<dyn Evaluatable>,
}
