use std::ops::{Index, IndexMut};

use crate::{
    element::{node::Node, Element, ElementIndex},
    library::Library,
};
#[derive(Debug, Clone)]
pub(crate) struct Elements<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    elements: Vec<Node<T>>,
}

impl<T> Elements<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    pub(crate) fn push_node(&mut self, node: impl Into<Node<T>>) -> ElementIndex {
        let index = self.elements.len();
        self.elements.push(node.into());
        ElementIndex(index)
    }
}
impl<T> Index<ElementIndex> for Elements<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    type Output = Node<T>;

    fn index(&self, index: ElementIndex) -> &Self::Output {
        let index = index.0;
        &self.elements[index]
    }
}
impl<T> IndexMut<ElementIndex> for Elements<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    fn index_mut(&mut self, index: ElementIndex) -> &mut Self::Output {
        let index = index.0;
        &mut self.elements[index]
    }
}
impl<T> Default for Elements<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    fn default() -> Self {
        Self {
            elements: Default::default(),
        }
    }
}
