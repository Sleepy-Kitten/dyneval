use crate::{error::Error, value::Value};

use dyneval_derive::*;
#[inline]
fn sqrt(values: [Value; 1]) -> Result<Value, Error> {
    let value = values[0];
    let result = match value {
        Value::Int(int) => (int as f64).sqrt(),
        Value::Float(float) => float.sqrt(),
    };
    Ok(Value::Float(result))
}
#[inline]
fn print(values: [Value; 1]) -> Result<Value, Error> {
    let value = values[0];
    println!("{value}");
    Ok(value)
}
pub enum Test {}
use crate::library::Library;
library! {
    Std; [];
    [
        fn print(num: i64) -> Result<Value, Error> {
            todo!()
        },
    ]
}
/*
functions!(
    Std: std;
    [];
    [
        Sqrt: sqrt(1),
        Print: print(1); false
        //Log: log(2),
        //Ln: ln(1),
        //Abs: abs(1)
        ]);
*/
