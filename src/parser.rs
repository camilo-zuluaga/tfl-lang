use core::fmt;
use std::{iter::Peekable, vec::IntoIter};

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
    #[cfg(test)]
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
            Formula::Implies(l, r) => write!(f, "({l} →  {r})"),
            Formula::Iff(l, r) => write!(f, "({l} ↔  {r})"),
        }
    }
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

// we opted for a struct since we will be doing recursive descent
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Formula, ParseError> {
        // we have to check for left overs here
        // for example: (P -> ~Q) R would be parsed and returned to (P -> ~Q) without check
        // which should not happen
        let formula = self.sentence()?;
        match self.advance() {
            None => Ok(formula),
            Some(t) => Err(ParseError::TrailingTokens(t)),
        }
    }

    fn advance(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn expect(&mut self, want: Token) -> Result<(), ParseError> {
        match self.advance() {
            Some(t) if t == want => Ok(()),
            found => Err(ParseError::Expected { want, found }),
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
                    found => Err(ParseError::ExpectedConnective(found)),
                }
            }

            found => Err(ParseError::UnexpectedStart(found)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    Expected { want: Token, found: Option<Token> },
    UnexpectedStart(Option<Token>),
    ExpectedConnective(Option<Token>),
    TrailingTokens(Token),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Expected {
                want,
                found: Some(c),
            } => write!(f, "want {want}, found {c}"),
            ParseError::Expected { want, found: None } => write!(f, "want {want}, found nothing"),
            ParseError::UnexpectedStart(Some(t)) => write!(f, "expected a formula, found {t}"),
            ParseError::UnexpectedStart(None) => write!(f, "expected a formula, but input ended"),
            ParseError::ExpectedConnective(Some(t)) => {
                write!(f, "expected a connective, found {t}")
            }
            ParseError::ExpectedConnective(None) => {
                write!(f, "expected a connective, but input ended")
            }
            ParseError::TrailingTokens(t) => write!(f, "unexpected token after formula: {t}"),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::*;
    use Formula as F;

    const LONG_TFL_SENTENCE: &str = "(((P →  Q) ∧ ¬R) ∧ (Q ∨ R))";

    #[test]
    fn prints_implies_with_negation() {
        let f = F::implies(F::atom("P"), F::not(F::atom("Q")));
        assert_eq!(f.to_string(), "(P →  ¬Q)")
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
        assert_eq!(f.to_string(), LONG_TFL_SENTENCE)
    }

    #[test]
    fn lexer_and_parser_compose() {
        let tokens = tokenize("(P -> ~Q)").unwrap(); // lexer: string  → tokens
        let tree = Parser::new(tokens).parse().unwrap(); // parser: tokens → tree
        assert_eq!(tree, F::implies(F::atom("P"), F::not(F::atom("Q"))));
    }

    #[test]
    fn lexer_and_parser_compose_long() {
        let tokens = tokenize(LONG_TFL_SENTENCE).unwrap();
        let tree = Parser::new(tokens).parse().unwrap();
        assert_eq!(
            tree,
            F::and(
                F::and(F::implies(F::atom("P"), F::atom("Q")), F::not(F::atom("R"))),
                F::or(F::atom("Q"), F::atom("R")),
            )
        );
    }
}
