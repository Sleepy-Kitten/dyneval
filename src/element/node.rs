use smallvec::SmallVec;

use crate::{library::Library, value::Value, variables::VariableIndex};

use super::{
    token::{Operator},
    ElementIndex,
};
#[derive(Debug, Clone)]
pub(crate) enum Node<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    Instruction {
        operator: Operator,
        lhs: ElementIndex,
        rhs: ElementIndex,
    },
    Literal(Value),
    Variable(VariableIndex),
    Function {
        function: T,
        args: SmallVec<[usize; T::MAX_ARGS]>,
    },
}
