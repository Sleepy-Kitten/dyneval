use nom::{
    branch::alt,
    character::complete::{char, i64, one_of, satisfy},
    combinator::{cond, map_opt, opt, recognize},
    multi::{many0, many1},
    sequence::{terminated, tuple},
    Err, IResult, ParseTo, Parser,
};

use crate::{element::node::Node, error::Error, library::Library, value::Value};

use super::Expression;

impl<T> Expression<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    pub fn parse_nom(&mut self) -> Result<(), Error> {
        Ok(())
    }
    fn parse_expression(&mut self, input: &str) -> IResult<&str, usize> {
        Err(nom::Err::Incomplete(nom::Needed::Unknown))
    }
    fn parse_expression_simple(&mut self, input: &str) -> IResult<&str, usize> {
        todo!()
    }
}
#[test]
fn nom_parse() {}
fn operand<T>(input: &str) -> IResult<&str, Node<T>>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    alt(())
}
fn identifier(input: &str) -> IResult<&str, &str> {
    let test = satisfy(char::is_alphabetic);
    recognize(tuple((test, nom::character::complete::alphanumeric0)))(input)
}
fn value(input: &str) -> IResult<&str, Value> {
    alt((i64.map(Value::Int), f64.map(Value::Float)))(input)
}
fn f64(input: &str) -> IResult<&str, f64> {
    map_opt(recognize(tuple((decimal, char('.'), opt(decimal)))), |s| {
        ParseTo::parse_to(&s)
    })(input)
}
fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}
