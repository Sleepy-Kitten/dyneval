use crate::{value::Value, error::Error};

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