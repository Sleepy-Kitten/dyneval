use std::ops::{Add, Div, Mul, Rem, Sub};

use crate::value::Value;

#[derive(Debug, Clone)]
pub(crate) struct Token {
    token_kind: TokenKind,
    start: usize,
    end: usize,
}
impl<'a> Token {
    pub fn new(token_kind: TokenKind, start: usize) -> Self {
        Token {
            token_kind,
            start,
            end: start + 1,
        }
    }
    pub fn kind(&self) -> TokenKind {
        self.token_kind
    }
    /// Set the token's token kind.
    pub(crate) fn set_kind(&mut self, token_kind: impl Into<TokenKind>) {
        self.token_kind = token_kind.into();
    }
    pub(crate) fn inc_end(&mut self) {
        self.end += 1;
    }
    pub(crate) fn set_inc(&mut self, token_kind: impl Into<TokenKind>) {
        self.token_kind = token_kind.into();
        self.end += 1;
    }
    pub(crate) fn new_neg_zero() -> Self {
        Self {
            token_kind: TokenKind::Special(Special::NegZero),
            start: 0,
            end: 0,
        }
    }
    pub(crate) fn slice<'b>(&'a self, string: &'b str) -> &'b str {
        &string[self.start..self.end]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Literal(Literal),
    Bracket(Bracket),
    Identifier(Identifier),
    Operator(Operator),
    Special(Special),
}
impl From<Literal> for TokenKind {
    fn from(literal: Literal) -> Self {
        Self::Literal(literal)
    }
}
impl From<Bracket> for TokenKind {
    fn from(bracket: Bracket) -> Self {
        Self::Bracket(bracket)
    }
}
impl From<Identifier> for TokenKind {
    fn from(identifier: Identifier) -> Self {
        Self::Identifier(identifier)
    }
}
impl From<Operator> for TokenKind {
    fn from(operator: Operator) -> Self {
        Self::Operator(operator)
    }
}
impl From<Special> for TokenKind {
    fn from(special: Special) -> Self {
        Self::Special(special)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Literal {
    Int,
    Float,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Bracket {
    Opened,
    Closed,
}
impl Bracket {
    pub fn weight(&self) -> i16 {
        match self {
            Self::Opened => 100,
            Self::Closed => -100,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Identifier {
    Function,
    Variable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Special {
    NamespacePartial,
    Namespace,
    NegZero,
    Comma,
}

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
