use crate::{error::Error, value::Value};
use crate::library::Library;
use dyneval_derive::*;

library! {
    Std; [];
    [
        fn print(val: Value) -> Result<Value, Error> {
            println!("{val}");
            Ok(val)
        },
    ]
}