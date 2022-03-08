use std::cmp::Ordering::*;

use smallvec::SmallVec;

use crate::{
    element::{
        node::Node,
        token::{
            Bracket::*, Identifier::*, Literal::*, Operator::*, Special::*, Token, TokenKind,
            TokenKind::*,
        },
        Element, ElementIndex,
    },
    error::Error,
    function::Function,
    value::Value,
    variables::Variables,
};

pub mod lex;
pub mod parse;


#[derive(Debug, Clone)]
pub struct ExpressionStorage<T>
where
    T: Function<T>,
    [(); T::MAX_ARGS]:,
{
    elements: Vec<Element<T>>,
    variables: Variables,
}
impl<T> Default for ExpressionStorage<T>
where
    T: Function<T>,
    [(); T::MAX_ARGS]:,
{
    fn default() -> Self {
        Self {
            elements: Default::default(),
            variables: Default::default(),
        }
    }
}

/// An `Expression` which stores the original expression string and the compiled version of that string.
/// This allows the expression to be evaluated multiple times without the overhead of being parsed again
///
#[derive(Debug, Clone)]
pub struct Expression<T>
where
    T: Function<T>,
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
    T: Function<T>,
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
    T: Function<T>,
    [(); T::MAX_ARGS]:,
{
    pub fn new(expression: String) -> Self {
        Self {
            string: expression,
            ..Default::default()
        }
    }
}
