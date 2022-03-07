#![feature(generic_const_exprs)]

use value::Value;

pub mod function;
pub mod value;
pub mod error;
pub mod variables;
pub mod element;
mod small_string;
pub mod expression;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
fn main() {
    let a = Value::Int(2);
}