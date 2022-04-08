use std::{collections::HashMap, ops::Index};

use smallvec::SmallVec;

use crate::{small_string::SmallString, value::Value};
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VariableIndex(pub(crate) usize);
#[derive(Debug, Default, Clone)]
pub(crate) struct Variables {
    identifiers: HashMap<SmallString<16>, usize>,
    values: SmallVec<[Value; 4]>,
}

impl Variables {
    pub fn clear(&mut self) {
        self.values.clear();
        self.values.clear();
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            identifiers: HashMap::with_capacity(capacity),
            values: SmallVec::with_capacity(capacity),
        }
    }
    pub(crate) fn push(&mut self, identifier: &str) {
        self.identifiers
            .insert(identifier.into(), self.values.len());
        self.values.push(Value::Int(0));
    }
    pub(crate) fn find_or_set(&mut self, identifier: &str) -> VariableIndex {
        let index = match self.identifiers.get(identifier) {
            Some(index) => *index,
            None => {
                self.push(identifier);
                self.values.len() - 1
            }
        };
        VariableIndex(index)
    }
}
impl Index<VariableIndex> for Variables {
    type Output = Value;

    fn index(&self, index: VariableIndex) -> &Self::Output {
        let index = index.0;
        &self.values[index]
    }
}
