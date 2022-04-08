use nom::{
    branch::alt,
    character::complete::{char, i64, one_of, satisfy},
    combinator::{cond, map, map_opt, opt, recognize},
    multi::{many0, many1, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    Err, IResult, ParseTo, Parser,
};
use smallvec::SmallVec;

use crate::{
    element::{
        node::{Function, Instruction, Node},
        token::Operator,
        Element, ElementIndex,
    },
    error::Error,
    library::{std::Std, Library},
    value::Value,
};

use super::{Expression, ExpressionStorage};
impl<'a, 'b, T> ExpressionStorage<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    pub(crate) fn parse_expression_delimited(&'a mut self, input: &'b str) -> IResult<&'b str, ElementIndex> {
        let expression = |i| self.parse_expression_delimited(i);

        let result = delimited(char('('), expression, char(')')).parse(input);
        let (input, index) = result.or_else(|_| self.parse_expression_simple(input))?;

        Ok((input, index))
    }
    fn parse_expression_simple(&'a mut self, input: &'b str) -> IResult<&'b str, ElementIndex> {
        let (input, index) = self
            .operand(input)
            .and_then(|(input, _)| {
                self.operator(input)
                    .map(|(input, index)| (input, Some(index)))
            })
            .or_else(|_| {
                let operand = |i| self.operand(i);
                nom::combinator::rest
                    .and(operand)
                    .parse(input)
                    .map(|(input, _)| (input, None))
            })?;
        if let Some(index) = index {
            self.parse_expression_delimited(input)
        } else {
            Ok((input, ElementIndex(0)))
        }
    }
    fn operator(&'a mut self, input: &'b str) -> IResult<&'b str, ElementIndex> {
        let (input, operator) = alt((
            char('+').map(|_| Operator::Add),
            char('-').map(|_| Operator::Sub),
            char('*').map(|_| Operator::Mul),
            char('/').map(|_| Operator::Div),
            char('%').map(|_| Operator::Rem),
            char('^').map(|_| Operator::Pow),
        ))(input)?;

        let instruction = Instruction {
            operator,
            lhs: ElementIndex(0),
            rhs: ElementIndex(0),
        };
        let index = self.elements.push_node(instruction);

        Ok((input, index))
    }
    fn operand(&'a mut self, input: &'b str) -> IResult<&'b str, ElementIndex> {
        self.literal(input)
            .or_else(|_| self.function(input))
            .or_else(|_| self.variable(input))
    }

    fn variable(&'a mut self, input: &'b str) -> IResult<&'b str, ElementIndex> {
        let (input, ident) = identifier(input)?;
        let index = self.variables.find_or_set(ident);
        let node = Node::Variable(index);

        let index = self.elements.push_node(node);
        Ok((input, index))
    }
    fn function(&'a mut self, input: &'b str) -> IResult<&'b str, ElementIndex> {
        let expression = |i| self.parse_expression_delimited(i);

        let (input, (ident, args)) = pair(
            identifier,
            delimited(char('('), separated_list0(char(','), expression), char(')')),
        )(input)?;

        let function = Function {
            function: T::from_string(&[], ident).unwrap(),
            args: SmallVec::from_vec(args),
        };

        let index = self.elements.push_node(function);
        Ok((input, index))
    }
    fn literal(&'a mut self, input: &'b str) -> IResult<&'b str, ElementIndex> {
        let (input, value) = alt((i64.map(Value::Int), f64.map(Value::Float)))(input)?;

        let index = self.elements.push_node(value);
        Ok((input, index))
    }
}
#[test]
fn nom_parse() {
    let mut expression = Expression::<Std>::new(String::from("sqrt(2)"));
    expression.parse_nom();
    dbg!(&expression);
}

fn literal(input: &str) -> IResult<&str, Value> {
    alt((i64.map(Value::Int), f64.map(Value::Float)))(input)
}
fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        satisfy(char::is_alphabetic),
        nom::character::complete::alphanumeric0,
    )))(input)
}

fn f64(input: &str) -> IResult<&str, f64> {
    map_opt(recognize(tuple((decimal, char('.'), opt(decimal)))), |s| {
        ParseTo::parse_to(&s)
    })(input)
}
fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}
