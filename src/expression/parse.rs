use std::{cmp::Ordering, fmt::Debug};

use smallvec::SmallVec;

use crate::{
    element::{
        node::Node,
        token::{
            Identifier::{self, *},
            Literal::*,
            Special::Comma,
            Special::*,
            TokenKind::*,
        },
        Element, ElementIndex,
    },
    error::Error,
    library::Library,
    value::Value,
};

use super::Expression;

#[derive(Debug)]
struct NodeInfo<'a, T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    index: ElementIndex,
    node: &'a mut Node<T>,
    weight: i16,
}
#[derive(Debug)]
struct IndexWeight {
    weight: i16,
    index: ElementIndex,
}

impl IndexWeight {
    pub fn assign_lower<'a, T>(&mut self, other: &NodeInfo<'a, T>)
    where
        T: Library<T>,
        [(); T::MAX_ARGS]:,
    {
        if other.weight <= self.weight {
            self.weight = other.weight;
            self.index = other.index;
        }
    }
}

impl<T> Expression<T>
where
    T: Library<T>,
    [(); T::MAX_ARGS]:,
{
    /// lexes the [`Expression`] string into [`Token`]s
    pub(crate) fn to_tokens(&mut self) -> Result<&mut Self, Error> {
        for (index, &chr) in self.string.as_bytes().iter().enumerate() {
            self.storage.push(index, chr)?;
        }
        Ok(self)
    }
    /// parses the [`Token`]s into [`Node`]s, containing actual data
    pub(crate) fn to_nodes(&mut self) -> Result<&mut Self, Error> {
        let mut namespaces = SmallVec::<[&str; 4]>::new();
        for element in &mut self.storage.elements {
            match element {
                Element::Token(token) => match token.kind() {
                    Literal(l) => {
                        let value = match l {
                            Float => Value::Float(token.slice(&self.string).parse::<f64>()?),
                            Int => Value::Int(token.slice(&self.string).parse::<i64>()?),
                            _ => unimplemented!(),
                        };
                        *element = Element::Node(Node::Literal(value));
                    }
                    Operator(o) => {
                        *element = Element::Node(Node::Instruction {
                            operator: o,
                            lhs: ElementIndex::new(0),
                            rhs: ElementIndex::new(0),
                        })
                    }
                    Identifier(Identifier::Function) => {
                        let identifier = token.slice(&self.string);
                        let function = <T as Library<T>>::from_string(&namespaces, identifier)?;
                        *element = Element::Node(Node::Function {
                            function,
                            args: SmallVec::new(),
                        })
                    }
                    Bracket(_) | Special(Comma) => (),
                    Special(Namespace) => namespaces.push(token.slice(&self.string)),
                    Special(NamespacePartial) => return Err(Error::UnexpectedToken),
                    Special(NegZero) => *element = Element::Node(Node::Literal(Value::Int(0))),
                    Identifier(Identifier::Variable) => {
                        let identifier = token.slice(&self.string);
                        let index = self.storage.variables.find_or_set(identifier);
                        *element = Element::Node(Node::Variable(index))
                    }
                },
                Element::Node(_) => (),
            }
        }
        drop(namespaces);
        Ok(self)
    }
    /// Set the left and right [`ElementIndex`] for each operator [`Node`]
    pub(crate) fn set_indices(&mut self) -> Result<&mut Self, Error> {
        // index and weight node with the lowest weight
        let mut lowest_weight = IndexWeight {
            weight: i16::MAX,
            index: ElementIndex::new(0),
        };
        let mut bracket_weight = 0;
        let mut iter = self
            .storage
            .elements
            .iter_mut()
            .enumerate()
            .map(|(index, element)| (ElementIndex::new(index), element))
            // filter out all nodes which are not operator/function nodes
            .filter_map(|(index, element)| match element {
                Element::Node(node) => match node {
                    Node::Instruction { operator, lhs, rhs } => {
                        // set operand indices to neighbor nodes
                        *lhs = index - ElementIndex::new(1);
                        *rhs = index + ElementIndex::new(1);
                        let weight = operator.weight() + bracket_weight;
                        let info = NodeInfo {
                            index,
                            node,
                            weight,
                        };
                        Some(info)
                    }
                    Node::Function { .. } => {
                        let info = NodeInfo {
                            index,
                            node,
                            weight: bracket_weight,
                        };
                        Some(info)
                    }
                    _ => None,
                },
                Element::Token(token) => {
                    if let Bracket(bracket) = token.kind() {
                        bracket_weight += bracket.weight();
                    }
                    None
                }
            })
            .peekable();
        loop {
            let next = iter.next();
            let peek = iter.peek_mut();
            match (next, peek) {
                (Some(next), Some(peek)) => {
                    let ordering = next.weight.cmp(&peek.weight);

                    match ordering {
                        Ordering::Equal => {
                            if let Node::Instruction { lhs, .. } = peek.node {
                                *lhs = next.index
                            }
                            lowest_weight.assign_lower(peek)
                        }
                        Ordering::Greater => {
                            if let Node::Instruction { lhs, .. } = peek.node {
                                *lhs = next.index;
                            }
                            lowest_weight.assign_lower(peek)
                        }
                        Ordering::Less => {
                            if let Node::Instruction { rhs, .. } = next.node {
                                *rhs = peek.index;
                            }
                            lowest_weight.assign_lower(&next)
                        }
                    }
                }
                (Some(next), None) => lowest_weight.assign_lower(&next),
                _ => break,
            }
        }
        self.root = Some(lowest_weight.index);
        Ok(self)
    }
    pub fn compile(&mut self) -> Result<(), Error> {
        self.to_tokens()?;
        self.to_nodes()?;
        self.set_indices()?;
        Ok(())
    }
}
