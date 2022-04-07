use smallvec::SmallVec;

use crate::{
    element::{node::Node, Element, ElementIndex},
    error::Error,
    library::Library,
    value::Value,
    variables::Variables,
};

pub mod lex;
pub mod parse;
mod parse_nom;
mod parse_mixed;

#[derive(Debug, Clone)]
pub struct ExpressionStorage<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    elements: Vec<Element<T>>,
    variables: Variables,
}
impl<T> Default for ExpressionStorage<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    fn default() -> Self {
        Self {
            elements: Default::default(),
            variables: Default::default(),
        }
    }
}
impl<T> ExpressionStorage<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    pub(crate) fn get_element(&self, index: ElementIndex) -> &Element<T> {
        &self.elements[index.0]
    }
    pub(crate) fn push_node(&mut self, node: Node<T>) -> ElementIndex {
        let index = self.elements.len();
        self.elements.push(Element::Node(node));
        ElementIndex(index)
    }
}

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
    pub fn new(expression: String) -> Self {
        Self {
            string: expression,
            ..Default::default()
        }
    }
    pub fn set_expression(&mut self, expression: String) {
        self.string = expression;
    }
    pub fn eval(&self) -> Result<Value, Error> {
        match self.root {
            Some(index) => self.eval_recursive(index),
            None => Err(Error::NotCompiled),
        }
    }
    fn eval_recursive(&self, index: ElementIndex) -> Result<Value, Error> {
        if let Element::Node(n) = &self.storage.get_element(index) {
            Ok(match n {
                Node::Instruction { operator, lhs, rhs } => {
                    operator.eval(self.eval_recursive(*lhs)?, self.eval_recursive(*rhs)?)
                }
                Node::Literal(value) => *value,
                Node::Variable(index) => self.storage.variables.get(*index),
                Node::Function { function, args } => {
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
}
