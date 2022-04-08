use std::ops::Index;

use smallvec::SmallVec;

use crate::{error::Error, expression::element::ElementIndex, library::Library, value::Value};

use self::expression_storage::{variables::Variables, ExpressionStorage};

mod element;
pub(crate) mod expression_storage;
mod parse;
/// An `Expression` which stores the original expression string and the compiled version of that string.
/// This allows the expression to be evaluated multiple times without the overhead of being parsed again
///
#[derive(Debug, Clone)]
pub struct Expression<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    /// Original expression string
    string: String,
    /// Root node of the expression
    root: Option<ElementIndex>,
    /// Storage of the expression, containing [`Variables`] and the compiled [`Element`]s
    storage: ExpressionStorage<T>,
}
impl<T> Default for Expression<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    fn default() -> Self {
        Self {
            string: Default::default(),
            root: Default::default(),
            storage: Default::default(),
        }
    }
}

impl<T> Expression<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    pub fn new(mut expression: String) -> Self {
        expression.remove_matches(' ');
        Self {
            string: expression,
            ..Default::default()
        }
    }
    pub fn set_expression(&mut self, expression: String) {
        self.string = expression;
    }
    pub fn eval(&self) -> Result<Value, Error> {
        /*
        match self.root {
            Some(index) => self.eval_recursive(index),
            None => Err(Error::NotCompiled),
        }
        */
        todo!()
    }
    /*
    fn eval_recursive(&self, index: ElementIndex) -> Result<Value, Error> {
        if let Element::Node(n) = &self.storage.get_element(index) {
            Ok(match n {
                Node::Instruction(Instruction { operator, lhs, rhs }) => {
                    operator.eval(self.eval_recursive(*lhs)?, self.eval_recursive(*rhs)?)
                }
                Node::Literal(value) => *value,
                Node::Variable(index) => self.storage.variables.get(*index),
                Node::Function(Function { function, args }) => {
                    let mut args_eval = SmallVec::<[Value; T::MAX_ARGS]>::new();
                    for arg in args.iter() {
                        args_eval.push(self.eval_recursive(*arg)?);
                    }
                    function.call(&args_eval)?
                }
            })
        } else {
            Err(Error::InvalidIndex)
        }
    }
    */
}
