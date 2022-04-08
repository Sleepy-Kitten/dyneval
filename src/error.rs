use std::{
    array::TryFromSliceError,
    convert::Infallible,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
    str::ParseBoolError,
};

#[derive(Debug)]
pub enum Error {
    UnkownCharacter(char),
    UnexpectedToken,
    NoIdentifierMatch,
    InvalidToken,
    InvalidNamespace,
    InvalidArg,
    InvalidIndex,
    InvalidVariable,
    InvalidType,
    NotCompiled,
    UnbalancedBracket,

    AlreadyCompiled,
    EmptyExpression,
    UnknownFunction,
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Self::InvalidToken
    }
}
impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self::InvalidToken
    }
}
impl From<ParseBoolError> for Error {
    fn from(_: ParseBoolError) -> Self {
        Self::InvalidToken
    }
}

impl From<TryFromSliceError> for Error {
    fn from(_: TryFromSliceError) -> Self {
        Self::InvalidArg
    }
}
impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self::InvalidArg
    }
}
impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
