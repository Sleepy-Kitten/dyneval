#![feature(generic_const_exprs)]
#![feature(try_blocks)]

use library::Library;

extern crate dyneval_derive;
//use library::std::test_print;
pub mod element;
pub mod error;
pub mod expression;
pub mod library;
mod small_string;
pub mod value;
pub mod variables;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
fn test_fn(a: u32, b: i32) {
    println!("a{a}, b{b}")
}
fn main() {
    dbg!(crate::library::std::Std::NAMESPACE);
    let array: [i8; 2] = [-11, 2];
    //let temp = test_fn(access_array!(array, u8, u8));
    let a = TryInto::<i16>::try_into(2_u8);
    //library::std::test_print();
    //call_with!(test_fn, array, i32, i32);
}
