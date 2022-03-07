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
    function::std::Function,
    value::Value,
    variables::Variables,
};

pub mod lex;
pub mod parse;
/// An `Expression` which stores the original expression string and the compiled version of that string.
/// This allows the expression to be evaluated multiple times without the overhead of being parsed again
/// 
/// 
#[derive(Debug, Default, Clone)]
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
#[derive(Debug, Default, Clone)]
pub struct ExpressionStorage<T>
where
    T: Function<T>,
    [(); T::MAX_ARGS]:,
{
    elements: Vec<Element<T>>,
    variables: Variables,
}