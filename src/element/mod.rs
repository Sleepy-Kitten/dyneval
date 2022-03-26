use std::ops::{Add, Sub};

use crate::library::Library;

use self::{node::Node, token::Token};

pub mod node;
pub mod token;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ElementIndex(pub(crate) usize);
impl ElementIndex {
    pub(crate) fn new(index: usize) -> Self {
        Self(index)
    }
}
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

#[derive(Debug, Clone)]
pub(crate) enum Element<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    Token(Token),
    Node(Node<T>),
}
