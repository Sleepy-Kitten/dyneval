extern crate test;
use super::{expression_storage, Expression, ExpressionStorage};
use crate::{
    error::Error,
    expression::element::{ElementIndex, Function, Instruction, Node, Operator::*},
    library::{std::Std, Library},
    value::Value,
};
use smallvec::SmallVec;
use std::str;

trait Recoverable {
    fn if_recoverable<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Self;
}
impl<T, E> Recoverable for Result<T, Option<E>> {
    fn if_recoverable<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        match self {
            Ok(ok) => Ok(ok),
            Err(None) => f(),
            Err(err) => Err(err),
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct IndexWeight {
    weight: i16,
    brackets: i16,
    index: ElementIndex,
}
impl Default for IndexWeight {
    fn default() -> Self {
        Self {
            weight: i16::MAX,
            brackets: 0,
            index: ElementIndex(0),
        }
    }
}
impl IndexWeight {
    fn assign_lower(&mut self, other: &Self) {
        if other.weight < self.weight {
            self.weight = other.weight;
            self.index = other.index;
        }
    }
    fn lower(self, other: Self) -> Self {
        if other.weight < self.weight {
            other
        } else {
            self
        }
    }
    fn add(&mut self, weight: i16) {
        self.weight += weight;
    }
    fn new(index: ElementIndex) -> Self {
        IndexWeight {
            index,
            ..Default::default()
        }
    }
    fn weight(&self) -> i16 {
        self.weight + self.brackets * 30
    }
}
type ParseResult<'a> = std::result::Result<(&'a [u8], IndexWeight), Option<Error>>;
impl<T> Expression<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    pub fn parse(&mut self) -> Result<(), Error> {
        let root = self.storage.parse(self.string.as_bytes())?;
        self.root = Some(root);
        Ok(())
    }
}
impl<'a, 'b, T> ExpressionStorage<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    fn parse(&'a mut self, input: &'b [u8]) -> Result<ElementIndex, Error> {
        let (_, result) = self.parse_expression_delimited(input, 0)?;
        Ok(result.index)
    }

    fn parse_expression_delimited(&'a mut self, input: &'b [u8], brackets: i16) -> ParseResult<'b> {
        if let Some(b'(') = input.first() {
            let input = &input[1..];

            let (input, index) = self.parse_expression_delimited(input, brackets + 1)?;

            if let Some(b')') = input.first() {
                let input = &input[1..];
                Ok((input, index))
            } else {
                Err(Some(Error::UnbalancedBracket))
            }
        } else {
            self.parse_expression_partial(input, brackets)
        }
    }
    fn parse_expression_partial(&'a mut self, input: &'b [u8], brackets: i16) -> ParseResult<'b> {
        let (input, operand) = self.parse_operand(input, brackets)?;
        let result = self.parse_operator(input);
        result
            .and_then(|(input, operator)| {
                let (input, expression) = self.parse_expression_delimited(input, brackets)?;

                let index = operator.lower(expression);

                let instruction = self.elements[operator.index].as_mut_instruction();
                instruction.lhs = operand.index;
                instruction.rhs = index.index;

                Ok((input, index))
            })
            .if_recoverable(|| self.parse_expression_delimited(input, brackets))
    }
    fn parse_operator(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
        if let Some(operator) = input.get(0) {
            let operator = match operator {
                b'+' => Add,
                b'-' => Sub,
                b'*' => Mul,
                b'/' => Div,
                b'%' => Rem,
                b'^' => Pow,
                _ => return Err(None),
            };
            let index = self.elements.push_node(Node::Instruction(Instruction {
                operator,
                lhs: Default::default(),
                rhs: Default::default(),
            }));
            let input = &input[1..];
            let mut index = IndexWeight::new(index);
            index.weight = operator.weight();
            Ok((input, index))
        } else {
            Err(Some(Error::InvalidToken))
        }
    }
    fn parse_operand(&'a mut self, input: &'b [u8], brackets: i16) -> ParseResult<'b> {
        self.parse_literal(input)
            .if_recoverable(|| self.parse_function(input, brackets))
            .if_recoverable(|| self.parse_identifier(input))
    }
    fn parse_literal(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
        if input.is_empty() {
            return Err(Some(Error::UnexpectedToken));
        }
        let digits = input
            .iter()
            .position(|chr| !chr.is_ascii_digit())
            .unwrap_or(input.len());

        let (digits, input) = input.split_at(digits);

        let (input, value) = if let Some(b'.') = input.first() {
            let decimal_digits = input[1..]
                .iter()
                .position(|chr| !chr.is_ascii_digit())
                .unwrap_or(input.len());

            let (slice, input) = input.split_at(decimal_digits);

            let float = str::from_utf8(slice)
                .unwrap()
                .parse()
                .map_err(|_| Some(Error::InvalidType))?;

            (input, Value::Float(float))
        } else {
            let int = str::from_utf8(digits).unwrap().parse().map_err(|_| None)?;

            (input, Value::Int(int))
        };

        let index = self.elements.push_node(Node::Literal(value));
        let index = IndexWeight::new(index);
        Ok((input, index))
    }
    fn parse_identifier(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
        if input.is_empty() {
            return Err(Some(Error::UnexpectedToken));
        }
        match identifier(input) {
            Ok((input, identifier)) => {
                let identifier = str::from_utf8(identifier).unwrap();

                let index = self.variables.find_or_set(identifier);
                let index = self.elements.push_node(Node::Variable(index));
                let index = IndexWeight::new(index);

                Ok((input, index))
            }
            Err(err) => Err(err),
        }
    }
    fn parse_function(&'a mut self, mut input: &'b [u8], brackets: i16) -> ParseResult<'b> {
        let mut namespaces = SmallVec::<[&str; 4]>::new();

        while let Ok((input_temp, namespace)) = namespace(input) {
            input = input_temp;
            namespaces.push(str::from_utf8(namespace).unwrap());
        }
        let result =
            identifier(input).map(|(input, identifier)| (input, identifier, input.first()));
        match result {
            Ok((input, identifier, Some(b'('))) => {
                let mut input = &input[1..];

                let identifier = str::from_utf8(identifier).unwrap();
                let function = T::from_string(&namespaces, identifier)?;
                let mut args = SmallVec::new();

                let input = loop {
                    let (input_temp, index) = self.parse_expression_delimited(input, brackets)?;
                    input = input_temp;
                    args.push(index.index);
                    match input.first() {
                        Some(b',') => input = &input[1..],
                        Some(b')') => break &input[1..],
                        _ => return Err(Some(Error::InvalidArg)),
                    }
                };
                let node = Node::Function(Function { function, args });
                let index = self.elements.push_node(node);
                let index = IndexWeight::new(index);
                Ok((input, index))
            }
            Err(Some(err)) => Err(Some(err)),
            _ if namespaces.is_empty() => Err(None),
            _ => Err(Some(Error::UnexpectedToken)),
        }
    }
}
fn namespace(input: &[u8]) -> Result<(&[u8], &[u8]), Option<Error>> {
    let (input, identifier) = identifier(input)?;
    if let Some(b"::") = input.get(0..2) {
        let input = &input[2..];
        Ok((input, identifier))
    } else {
        Err(None)
    }
}
fn identifier(input: &[u8]) -> Result<(&[u8], &[u8]), Option<Error>> {
    if input.is_empty() {
        return Err(Some(Error::UnexpectedToken));
    }
    let is_first_alphabetic = input.get(0).copied().map(|chr| chr.is_ascii_alphabetic());
    if let Some(true) = is_first_alphabetic {
        let characters = input
            .iter()
            .position(|chr| !chr.is_ascii_alphanumeric())
            .unwrap_or(input.len());

        let (slice, input) = input.split_at(characters);
        Ok((input, slice))
    } else {
        Err(None)
    }
}
#[test]
fn parse() {
    let a = [1, 2, 3, 4];
    let b = &a[0..=0];
    dbg!(Std::from_string(&["std"], "print"));
    dbg!(b);
    let mut expression = Expression::<Std>::new(String::from("std::print(std::print(3))"));
    dbg!(expression.parse());
    dbg!(&expression);
}
#[bench]
fn bench_parse(b: &mut test::Bencher) {
    let mut expression = Expression::<Std>::new(String::from("1*2+1^4"));
    dbg!(expression.parse());
    dbg!(&expression);
    b.iter(|| {
        expression.parse();
    })
}
