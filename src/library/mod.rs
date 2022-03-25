pub mod std;

use crate::{error::Error, value::Value};

pub trait Library<T>
where
    T: Library<T>,
{
    const NAMESPACE: &'static str;
    const MAX_ARGS: usize;
    fn from_string(namespaces: &[&str], identifier: &str) -> Result<T, Error>;
    fn call(&self, args: &[Value]) -> Result<Value, Error>;
    fn is_const(&self) -> bool {
        true
    }
}
enum STD {
    Tst
}
impl Library<STD> for STD {
    const NAMESPACE: &'static str = "";

    const MAX_ARGS: usize = 0;

    fn from_string(namespaces: &[&str], identifier: &str) -> Result<STD, Error> {
        match identifier {
            "test" => Ok(Self::Tst),
            _ => todo!()
        }
    }

    fn call(&self, args: &[Value]) -> Result<Value, Error> {
        todo!()
    }
}