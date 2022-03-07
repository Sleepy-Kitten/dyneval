use std::{borrow::Borrow, str::from_utf8_unchecked};

use smallvec::SmallVec;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SmallString<const LENGTH: usize> {
    vec: SmallVec<[u8; LENGTH]>,
}

impl<const LENGTH: usize> From<&str> for SmallString<LENGTH> {
    fn from(string: &str) -> Self {
        let bytes = string.as_bytes();
        Self { vec: bytes.into() }
    }
}

impl<const SIZE: usize> Borrow<str> for SmallString<SIZE> {
    fn borrow(&self) -> &str {
        unsafe { from_utf8_unchecked(&self.vec) }
    }
}
