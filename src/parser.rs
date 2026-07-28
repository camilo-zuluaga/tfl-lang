use core::fmt;
use std::{fmt::write, iter::Peekable, vec::IntoIter};

use crate::lexer::Token::{self};

#[derive(Debug, PartialEq)]
pub enum Formula {
    Atom(String),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Iff(Box<Formula>, Box<Formula>),
}

impl Formula {
    // helper methods to avoid typing box::new 300 times
    pub fn atom(name: &str) -> Formula {
        Formula::Atom(name.to_string())
    }
    pub fn not(f: Formula) -> Formula {
        Formula::Not(Box::new(f))
    }
    pub fn and(l: Formula, r: Formula) -> Formula {
        Formula::And(Box::new(l), Box::new(r))
    }
    pub fn or(l: Formula, r: Formula) -> Formula {
        Formula::Or(Box::new(l), Box::new(r))
    }
    pub fn implies(l: Formula, r: Formula) -> Formula {
        Formula::Implies(Box::new(l), Box::new(r))
    }
    pub fn iff(l: Formula, r: Formula) -> Formula {
        Formula::Iff(Box::new(l), Box::new(r))
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Formula::Atom(name) => write!(f, "{name}"),
            Formula::Not(inner) => write!(f, "¬{inner}"),
            Formula::And(l, r) => write!(f, "({l} ∧ {r})"),
            Formula::Or(l, r) => write!(f, "({l} ∨ {r})"),
            Formula::Implies(l, r) => write!(f, "({l} → {r})"),
            Formula::Iff(l, r) => write!(f, "({l} ↔ {r})"),
        }
    }
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn parse(t: Vec<Token>) -> Result<Formula, ParseError> {
        let pit = t.into_iter().peekable();
        let mut p = Parser { tokens: pit };
        p.sentence()
    }

    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    fn expect(&mut self, want: Token) -> Result<(), ParseError> {
        match self.advance() {
            Some(t) if t == want => Ok(()),
            _ => Err(ParseError::Expected()),
        }
    }

    fn sentence(&mut self) -> Result<Formula, ParseError> {
        match self.advance() {
            Some(Token::Atom(name)) => Ok(Formula::Atom(name)),

            Some(Token::Not) => {
                let inner = self.sentence()?;
                Ok(Formula::not(inner))
            }

            Some(Token::LeftParen) => {
                let left = self.sentence()?;
                let op = self.advance();
                let right = self.sentence()?;
                self.expect(Token::RightParen)?;

                match op {
                    Some(Token::And) => Ok(Formula::and(left, right)),
                    Some(Token::Or) => Ok(Formula::or(left, right)),
                    Some(Token::Implies) => Ok(Formula::implies(left, right)),
                    Some(Token::Iff) => Ok(Formula::iff(left, right)),
                    found => Err(ParseError::Expected()),
                }
            }

            found => Err(ParseError::Expected()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    Expected(),
}

#[cfg(test)]
mod tests {
    use crate::{lexer::tokenize, parser};

    use super::*;
    use Formula as F;

    #[test]
    fn prints_implies_with_negation() {
        let f = F::implies(F::atom("P"), F::not(F::atom("Q")));
        assert_eq!(f.to_string(), "(P → ¬Q)")
    }

    #[test]
    fn prints_double_negation() {
        let f = F::not(F::not(F::atom("A")));
        assert_eq!(f.to_string(), "¬¬A")
    }

    #[test]
    fn prints_nested_binary() {
        let f = F::and(
            F::and(F::implies(F::atom("P"), F::atom("Q")), F::not(F::atom("R"))),
            F::or(F::atom("Q"), F::atom("R")),
        );
        assert_eq!(f.to_string(), "(((P → Q) ∧ ¬R) ∧ (Q ∨ R))")
    }

    #[test]
    fn lexer_and_parser_compose() {
        let tokens = tokenize("(P -> ~Q)").unwrap(); // lexer: string  → tokens
        let tree = Parser::parse(tokens).unwrap(); // parser: tokens → tree
        assert_eq!(
            tree,
            Formula::implies(Formula::atom("P"), Formula::not(Formula::atom("Q")))
        );
    }
}
