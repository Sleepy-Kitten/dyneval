use crate::library::Library;

use self::{elements::Elements, variables::Variables};

pub mod elements;
pub mod variables;

#[derive(Debug, Clone)]
pub(crate) struct ExpressionStorage<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    pub elements: Elements<T>,
    pub variables: Variables,
}
impl<T> ExpressionStorage<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    pub(crate) fn clear(&mut self) {
        self.elements.clear();
        self.variables.clear();
    }
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
