#[macro_use]
pub mod define_macro;
pub mod std;

use crate::{error::Error, value::Value};

pub trait Function<T>
where
    T: Function<T>,
{
    const NAMESPACE: &'static str;
    const MAX_ARGS: usize;
    fn from_string(namespaces: &[&str], identifier: &str) -> Result<T, Error>;
    fn call(&self, args: &[Value]) -> Result<Value, Error>;
    fn is_const(&self) -> bool {
        true
    }
}
