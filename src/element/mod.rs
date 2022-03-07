use std::ops::{Add, Sub};

use crate::function::std::Function;

use self::{node::Node, token::Token};

pub mod node;
pub mod token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ElementIndex(pub(crate) usize);

impl Add for ElementIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl Sub for ElementIndex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
pub(crate) enum Element<T>
where
    T: Function<T>,
    [(); T::MAX_ARGS]:,
{
    Token(Token),
    Node(Node<T>),
}
