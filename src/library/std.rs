use crate::{error::Error, functions, value::Value};

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
make_answer!();
#[dyneval_derive::library]
pub mod test {
    fn testy(a: i64) -> crate::value::Value {
        crate::value::Value::Int(a)
    }
    fn uwu(a: f64, b: i64) -> crate::value::Value {
        todo!()
    }
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
