use std::fmt::Display;

use crate::error::Error;

/// Value type
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Value {
    Int(i64),
    Float(f64),
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(int) => int.fmt(f),
            Self::Float(float) => float.fmt(f),
        }
    }
}

impl From<i64> for Value {
    fn from(int: i64) -> Self {
        Self::Int(int)
    }
}
impl From<f64> for Value {
    fn from(float: f64) -> Self {
        Self::Float(float)
    }
}

impl TryInto<i64> for Value {
    type Error = Error;

    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Self::Int(int) => Ok(int),
            Self::Float(_) => Err(Error::InvalidType),
        }
    }
}
impl TryInto<f64> for Value {
    type Error = Error;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Self::Float(float) => Ok(float),
            Self::Int(_) => Err(Error::InvalidType),
        }
    }
}
