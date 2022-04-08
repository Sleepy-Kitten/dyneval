use std::ops::{Add, Div, Mul, Rem, Sub};

use smallvec::SmallVec;

use crate::{
    expression::expression_storage::variables::VariableIndex, library::Library, value::Value,
};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,

    Not,
    Or,
    Xor,
    And,

    Eq,
    NEq,
    GEq,
    Gt,
    LEq,
    Lt,
}

impl Operator {
    #[inline]
    pub(crate) fn weight(&self) -> i16 {
        match self {
            Self::Add => 20,
            Self::Sub => 20,
            Self::Mul => 21,
            Self::Div => 21,
            Self::Rem => 22,
            Self::Pow => 22,

            Self::Not => 1,
            Self::Eq => 2,
            Self::And => 3,
            Self::Or => 4,
            Self::Xor => 5,
            Self::NEq => 7,
            Self::Gt => 8,
            Self::GEq => 9,
            Self::Lt => 10,
            Self::LEq => 11,
        }
    }
    pub(crate) fn eval(&self, lhs: Value, rhs: Value) -> Value {
        match (lhs, rhs) {
            (Value::Float(lhs), Value::Float(rhs)) => Value::Float(self.eval_generic(lhs, rhs)),
            (Value::Int(lhs), Value::Int(rhs)) => Value::Int(self.eval_generic(lhs, rhs)),
            _ => unreachable!(),
        }
    }
    fn eval_generic<T>(self, lhs: T, rhs: T) -> T
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Rem<Output = T>,
    {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
            Self::Rem => lhs % rhs,
            Self::Pow => todo!(),
            _ => todo!(),
        }
    }
}
