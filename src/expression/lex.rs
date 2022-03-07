use crate::{
    element::{
        token::{
            Bracket::*, Identifier::*, Literal::*, Operator::*, Special::*, Token, TokenKind,
            TokenKind::*,
        },
        Element,
    },
    error::Error,
    function::std::Function,
};

use super::ExpressionStorage;

impl<T> ExpressionStorage<T>
where
    T: Function<T>,
    [(); T::MAX_ARGS]:,
{
    pub(crate) fn token_from(&mut self, token_kind: impl Into<TokenKind>, index: usize) {
        let token = Token::new(token_kind.into(), index);
        let element = Element::Token(token);
        self.elements.push(element);
    }
    pub(crate) fn new_token(&mut self, chr: u8, start: usize) -> Result<(), Error> {
        let token_kind = match chr {
            b'0'..=b'9' => Int.into(),

            b'a'..=b'z' | b'A'..=b'Z' => Variable.into(),

            b'+' => Add.into(),
            b'-' => Sub.into(),
            b'*' => Mul.into(),
            b'/' => Div.into(),
            b'%' => Rem.into(),
            b'^' => Pow.into(),
            b'=' => Eq.into(),
            b'!' => Not.into(),
            b'>' => Gt.into(),
            b'<' => Lt.into(),
            b'&' => And.into(),
            b'|' => Or.into(),
            b'#' => Xor.into(),

            b'(' => Opened.into(),
            b')' => Closed.into(),

            b'.' => Float.into(),

            _ => Err(Error::UnkownCharacter(chr as char))?,
        };
        let token = Token::new(token_kind, start);
        self.elements.push(Element::Token(token));
        Ok(())
    }
    pub(crate) fn push(&mut self, index: usize, chr: u8) -> Result<(), Error> {
        match self.elements.last_mut() {
            Some(Element::Token(token)) => {
                let kind = token.kind();
                match chr {
                    b' ' => (),
                    b':' => match kind {
                        Identifier(_) => token.set_kind(Namespace),
                        _ => Err(Error::UnexpectedToken)?,
                    },
                    b'0'..=b'9' => match kind {
                        Identifier(_) | Literal(_) => token.inc_end(),
                        _ => self.token_from(Variable, index),
                    },
                    b'a'..=b'z' | b'A'..=b'Z' => match kind {
                        Identifier(_) => token.inc_end(),
                        _ => self.token_from(Float, index),
                    },
                    b'.' => match kind {
                        Literal(Int) => token.set_inc(Int),
                        _ => self.token_from(Float, index),
                    },
                    b'+' | b'-' | b'*' | b'/' | b'%' | b'^' | b'&' | b'|' | b'!' | b'=' | b'<'
                    | b'>' | b'#' => match (chr, kind) {
                        (b'-', Operator(_) | Bracket(_)) => todo!(),
                        (b'>', Operator(Eq)) => token.set_inc(GEq),
                        (b'<', Operator(Eq)) => token.set_inc(LEq),
                        (b'!', Operator(Eq)) => token.set_inc(NEq),
                        _ => self.new_token(chr, index)?,
                    },
                    b'(' | b')' => match (chr, kind) {
                        (b'(', Identifier(_)) => {
                            token.set_kind(Function);
                            self.token_from(Opened, index);
                        }
                        (b'(', _) => self.token_from(Opened, index),
                        (b')', _) => self.token_from(Closed, index),
                        _ => unreachable!(),
                    },
                    b',' => self.token_from(Comma, index),
                    _ => Err(Error::UnexpectedToken)?,
                };
            }
            _ => match chr {
                b'-' => self.insert_neg(index),
                _ => self.new_token(chr, index)?,
            },
        };
        Ok(())
    }

    fn insert_neg(&mut self, index: usize) {
        self.elements.push(Element::Token(Token::new_neg_zero()));
        self.elements
            .push(Element::Token(Token::new(Sub.into(), index)));
    }
}
