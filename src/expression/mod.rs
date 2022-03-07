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

pub mod parse;
pub mod lex;
/// An `Expression` which stores the original string and the compiled version of that string
pub struct Expression<T>
where
    T: Function<T>,
    [(); T::MAX_ARGS]:,
{
    /// Original expression string
    string: String,
    /// Root node of the expression
    root: Option<ElementIndex>,

    storage: ExpressionStorage<T>,
}
pub struct ExpressionStorage<T>
where
    T: Function<T>,
    [(); T::MAX_ARGS]:,
{
    elements: Vec<Element<T>>,
    variables: Variables,
}


