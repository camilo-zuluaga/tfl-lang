use core::fmt;
use std::{iter::Peekable, vec::IntoIter};

use crate::{
    ast::Formula,
    lexer::Token::{self},
};

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
        // for example: (P -> ~Q) R would be parsed and returned as (P -> ~Q) without check,
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

    const LONG_TFL_SENTENCE: &str = "(((P → Q) ∧ ¬R) ∧ (Q ∨ R))";

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
