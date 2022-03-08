#![feature(generic_const_exprs)]
#![feature(try_blocks)]
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
fn test_fn(a: i32, b: i32) {
    println!("a{a}, b{b}")
}
fn main() {
    let array: [u8; 2] = [1,2];
    //let temp = test_fn(access_array!(array, u8, u8));
    let a = TryInto::<i16>::try_into(2_u8);
    let test: Result<(), std::convert::Infallible> = try {
        call_with!(array, test_fn, (i32, i32));
    };

    //call_with!(test_fn, array, i32, i32);
}