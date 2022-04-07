use std::str;

use crate::{
    element::{node::Node, token::Operator::*, ElementIndex},
    error::Error,
    library::Library,
    value::Value,
};

use super::ExpressionStorage;
type ParseResult<'a> = std::result::Result<(&'a [u8], ElementIndex), Error>;

impl<'a, 'b, T> ExpressionStorage<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    pub fn parse_operator(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
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
            let index = self.push_node(Node::Instruction {
                operator,
                lhs: Default::default(),
                rhs: Default::default(),
            });
            Ok((&input[1..], index))
        } else {
            Err(Error::InvalidToken)
        }
    }
    fn parse_literal(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
        if input.is_empty() {
            return Err(Error::UnexpectedToken);
        }
        let digits = input.iter().position(|chr| !chr.is_ascii_digit());

        if let Some(digits) = digits {
            let has_decimal = input.get(digits).copied().contains(&b'.');

            let (input, value) = if has_decimal {
                let decimal_digits = input[(digits + 1)..]
                    .iter()
                    .position(|chr| !chr.is_ascii_digit())
                    .unwrap_or(input.len() - 1);

                let slice = &input[..decimal_digits];
                let float = str::from_utf8(slice).unwrap().parse()?;

                (&input[decimal_digits..], Value::Float(float))
            } else {
                let slice = &input[..digits];
                let int = str::from_utf8(slice).unwrap().parse()?;

                (&input[digits..], Value::Int(int))
            };

            let index = self.push_node(Node::Literal(value));
            Ok((input, index))
        } else {
            Err(Error::UnexpectedToken)
        }
    }
    fn parse_identifier(&'a mut self, input: &'b [u8]) -> ParseResult<'b> {
        if input.is_empty() {
            return Err(Error::UnexpectedToken);
        }
        let is_first_alphabetic = input.get(0).copied().map(|chr| chr.is_ascii_alphabetic());
        if let Some(is_first_alphabetic) = is_first_alphabetic {
            if is_first_alphabetic {
                let characters = input
                    .iter()
                    .position(|chr| !chr.is_ascii_alphanumeric())
                    .unwrap_or(input.len() - 1);

                let slice = &input[..characters];
                let identifier = str::from_utf8(slice).unwrap();

                let index = self.variables.find_or_set(identifier);
                let index = self.push_node(Node::Variable(index));

                Ok((&input[characters..], index))
            } else {
                Err(Error::InvalidVariable)
            }
        } else {
            Err(Error::UnexpectedToken)
        }
    }
}
