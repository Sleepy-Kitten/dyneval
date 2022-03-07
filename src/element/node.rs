use smallvec::SmallVec;

use crate::{function::std::Function, value::Value, variables::VariableIndex};

use super::{
    token::{Bracket, Operator, Special},
    ElementIndex,
};
#[derive(Debug, Clone)]
pub(crate) enum Node<T>
where
    T: Function<T>,
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
