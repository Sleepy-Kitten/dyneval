use std::str;

use smallvec::SmallVec;

use crate::{
    element::{
        node::{Function, Instruction, Node},
        token::Operator::*,
        ElementIndex,
    },
    error::Error,
    library::{std::Std, Library},
    value::Value,
};

use super::{expression_storage, Expression, ExpressionStorage};
type ParseResult<'a> = std::result::Result<(&'a [u8], ElementIndex), Error>;

impl<'a, 'b, T> ExpressionStorage<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    pub(crate) fn parse_expression_mixed(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
        if let Some(b'(') = input.first() {
            let input = &input[1..];
            let (input, index) = self.parse_expression_mixed(input)?;

            if let Some(b')') = input.first() {
                Ok((input, index))
            } else {
                Err(Error::UnbalancedBracket)
            }
        } else {
            self.parse_expression_partial(input)
        }
    }
    fn parse_expression_partial(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
        let (input, operand) = self.parse_operand(input)?;

        if let Ok((input, operator)) = self.parse_operator(input) {
            let instruction = self.elements[operator].as_mut_instruction();
            instruction.lhs = operand;
            let (input, index) = self.parse_expression_mixed(input)?;
            Ok((input, index))
        } else {
            if false {
                self.parse_expression_mixed(input)?;
            }
            Ok((input, operand))
        }
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
                _ => return Err(Error::InvalidToken),
            };
            let index = self.elements.push_node(Node::Instruction(Instruction {
                operator,
                lhs: Default::default(),
                rhs: Default::default(),
            }));
            let input = &input[1..];
            Ok((input, index))
        } else {
            Err(Error::InvalidToken)
        }
    }
    fn parse_operand(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
        self.parse_literal(input)
            .or_else(|_| self.parse_function(input))
            .or_else(|_| self.parse_identifier(input))
    }
    fn parse_literal(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
        if input.is_empty() {
            return Err(Error::UnexpectedToken);
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

            let float = str::from_utf8(slice).unwrap().parse()?;

            (input, Value::Float(float))
        } else {
            let int = str::from_utf8(digits).unwrap().parse()?;

            (input, Value::Int(int))
        };

        let index = self.elements.push_node(Node::Literal(value));
        Ok((input, index))
    }
    fn parse_identifier(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
        if input.is_empty() {
            return Err(Error::UnexpectedToken);
        }
        if let Ok((input, identifier)) = identifier(input) {
            let identifier = str::from_utf8(identifier).unwrap();

            let index = self.variables.find_or_set(identifier);
            let index = self.elements.push_node(Node::Variable(index));

            Ok((input, index))
        } else {
            Err(Error::InvalidVariable)
        }
    }
    fn parse_function(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
        if let Ok((input, identifier)) = identifier(input) {
            let identifier = str::from_utf8(identifier).unwrap();

            let function = T::from_string(&[], identifier)?;
            let mut args = SmallVec::new();

            let input = if let Some(b'(') = input.first() {
                let mut input = &input[1..];
                loop {
                    let (input_temp, index) = self.parse_expression_mixed(input)?;
                    input = input_temp;
                    args.push(index);
                    match input.first() {
                        Some(b',') => input = &input[1..],
                        Some(b')') => break &input[1..],
                        _ => return Err(Error::InvalidArg),
                    }
                }
            } else {
                return Err(Error::UnexpectedToken);
            };

            let node = Node::Function(Function { function, args });
            let index = self.elements.push_node(node);
            Ok((input, index))
        } else {
            Err(Error::UnexpectedToken)
        }
    }
}

fn identifier(input: &[u8]) -> Result<(&[u8], &[u8]), Error> {
    if input.is_empty() {
        return Err(Error::UnexpectedToken);
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
        Err(Error::InvalidVariable)
    }
}
#[test]
fn parsy() {
    let a = [1, 2, 3, 4];
    let b = &a[0..=0];
    dbg!(b);
    let mut expression = Expression::<Std>::new(String::from("1+2+print((2))*(3+4)"));
    expression.parse_mixed();
    dbg!(&expression);
}
