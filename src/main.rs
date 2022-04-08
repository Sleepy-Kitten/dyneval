#![feature(generic_const_exprs)]
#![feature(try_blocks)]
#![feature(string_remove_matches)]

use expression::Expression;
use library::{std::Std, Library};

extern crate dyneval_derive;
//use library::std::test_print;
pub mod element;
pub mod error;
pub mod expression;
pub mod library;
mod small_string;
pub mod value;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

fn main() {
    let mut expression = Expression::<Std>::new("1+print(1)".to_owned());
    dbg!(&expression);
    dbg!(expression.eval());
    return ();
}
