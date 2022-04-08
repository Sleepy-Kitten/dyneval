use smallvec::SmallVec;

use crate::{
    expression::expression_storage::variables::VariableIndex, library::Library, value::Value,
};

use super::{token::Operator, ElementIndex};
#[derive(Debug, Clone)]
pub(crate) enum Node<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    Instruction(Instruction),
    Literal(Value),
    Variable(VariableIndex),
    Function(Function<T>),
}
#[derive(Debug, Clone)]
pub(crate) struct Instruction {
    pub operator: Operator,
    pub lhs: ElementIndex,
    pub rhs: ElementIndex,
}
#[derive(Debug, Clone)]
pub(crate) struct Function<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    pub function: T,
    pub args: SmallVec<[ElementIndex; T::MAX_ARGS]>,
}

macro_rules! impl_from_node {
    ($item:ty, $variant:ident) => {
        impl<T> From<$item> for Node<T>
        where
            T: Library<T>,
            [(); T::MAX_ARGS]:,
        {
            fn from(value: $item) -> Node<T> {
                Node::$variant(value)
            }
        }
    };
}
macro_rules! impl_to_node {
    ($item:ty, $variant:ident) => {
        impl<T> TryFrom<Node<T>> for $item
        where
            T: Library<T>,
            [(); T::MAX_ARGS]:,
        {
            type Error = ();

            fn try_from(node: Node<T>) -> Result<Self, Self::Error> {
                match node {
                    Node::$variant(value) => Ok(value),
                    _ => Err(()),
                }
            }
        }
    };
}
macro_rules! impl_node_convert {
    ($item:ty, $variant:ident) => {
        impl_from_node! {$item, $variant}
        impl_to_node! {$item, $variant}
    };
}
macro_rules! impl_node_as {
    ($name:ident, $item:ty, $variant:ident) => {
        pub(crate) fn $name(&self) -> &$item {
            match self {
                Self::$variant(value) => &value,
                _ => panic!(),
            }
        }
    };
    ($name:ident, $item:ty, $variant:ident, mut) => {
        pub(crate) fn $name(&mut self) -> &mut $item {
            match self {
                Self::$variant(value) => value,
                _ => panic!(),
            }
        }
    };
}
impl<T> Node<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    impl_node_as! {as_instruction, Instruction, Instruction}
    impl_node_as! {as_variable, VariableIndex, Variable}
    impl_node_as! {as_value, Value, Literal}
    impl_node_as! {as_function, Function<T>, Function}

    impl_node_as! {as_mut_instruction, Instruction, Instruction, mut}
    impl_node_as! {as_mut_variable, VariableIndex, Variable, mut}
    impl_node_as! {as_mut_value, Value, Literal, mut}
    impl_node_as! {as_mut_function, Function<T>, Function, mut}
}
impl_node_convert! {Instruction, Instruction}
impl_node_convert! {VariableIndex, Variable}
impl_node_convert! {Value, Literal}
impl_node_convert! {Function<T>, Function}
