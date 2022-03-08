use crate::{
    element::{
        token::{
            Bracket::*, Identifier::*, Literal::*, Operator::*, Special::*, Token, TokenKind,
            TokenKind::*,
        },
        Element,
    },
    error::Error,
    function::Function,
};

use super::ExpressionStorage;

impl<T> ExpressionStorage<T>
where
    T: Function<T>,
    [(); T::MAX_ARGS]:,
{
    /// inserts a new token into the storage from kind
    pub(crate) fn token_from(&mut self, token_kind: impl Into<TokenKind>, index: usize) {
        let token = Token::new(token_kind.into(), index);
        let element = Element::Token(token);
        self.elements.push(element);
    }
    /// inserts a new token into the storage from char
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

            _ => return Err(Error::UnkownCharacter(chr as char)),
        };
        let token = Token::new(token_kind, start);
        self.elements.push(Element::Token(token));
        Ok(())
    }
    /// lex char into token, depending on the last token
    pub(crate) fn push(&mut self, index: usize, chr: u8) -> Result<(), Error> {
        match self.elements.last_mut() {
            Some(Element::Token(token)) => {
                let kind = token.kind();
                match (chr, kind) {
                    // special
                    (b' ', _) => (),
                    (b':', Identifier(_)) => token.set_kind(Namespace),
                    // literal, variable
                    (b'.', Literal(Int)) => token.set_inc(Float),
                    (b'.', _) => self.token_from(Float, index),
                    (b'0'..=b'9', Identifier(_) | Literal(_)) => token.inc_end(),
                    (b'0'..=b'9', _) => self.token_from(Variable, index),
                    (b'a'..=b'z' | b'A'..=b'Z', Identifier(_)) => token.inc_end(),
                    (b'a'..=b'z' | b'A'..=b'Z', _) => self.token_from(Float, index),
                    // function
                    (b'(', Identifier(_)) => {
                        token.set_kind(Function);
                        self.token_from(Opened, index);
                    }
                    // operators, others
                    (b'-', Operator(_)) => self.insert_neg(index),
                    (b'>', Operator(Eq)) => token.set_inc(GEq),
                    (b'<', Operator(Eq)) => token.set_inc(LEq),
                    (b'!', Operator(Eq)) => token.set_inc(NEq),
                    (b'+', _) => self.token_from(Add, index),
                    (b'-', _) => self.token_from(Sub, index),
                    (b'*', _) => self.token_from(Mul, index),
                    (b'/', _) => self.token_from(Div, index),
                    (b'%', _) => self.token_from(Rem, index),
                    (b'^', _) => self.token_from(Pow, index),
                    (b'&', _) => self.token_from(And, index),
                    (b'|', _) => self.token_from(Or, index),
                    (b'#', _) => self.token_from(Xor, index),
                    (b'!', _) => self.token_from(Not, index),
                    (b'=', _) => self.token_from(Eq, index),
                    (b'<', _) => self.token_from(Lt, index),
                    (b'>', _) => self.token_from(Gt, index),
                    (b'(', _) => self.token_from(Opened, index),
                    (b')', _) => self.token_from(Closed, index),
                    (b',', _) => self.token_from(Comma, index),
                    _ => return Err(Error::UnexpectedToken),
                };
            }
            Some(Element::Node(_)) => return Err(Error::AlreadyCompiled),
            // first char
            _ => match chr {
                b'-' => self.insert_neg(index),
                _ => self.new_token(chr, index)?,
            },
        };
        Ok(())
    }

    /// inserts special token for cases like 4  + -2
    fn insert_neg(&mut self, index: usize) {
        self.elements.push(Element::Token(Token::new_neg_zero()));
        self.elements
            .push(Element::Token(Token::new(Sub.into(), index)));
    }
}
