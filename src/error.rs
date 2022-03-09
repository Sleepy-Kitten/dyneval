use std::{num::{ParseFloatError, ParseIntError, TryFromIntError}, array::TryFromSliceError, str::ParseBoolError, convert::Infallible};

#[derive(Debug)]
pub enum Error {
    UnkownCharacter(char),
    UnexpectedToken,
    NoIdentifierMatch,
    InvalidToken,
    InvalidNamespace,
    InvalidArgs,
    InvalidIndex,
    InvalidVariable,
    InvalidType,
    NotCompiled,
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
        Self::InvalidArgs
    }
}
impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Self::InvalidArgs
    }
}
impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}