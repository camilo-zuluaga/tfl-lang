use core::fmt;
use std::{array::IntoIter, fmt::write, iter::Peekable};

use crate::lexer::Token::{self, Atom, Not};

pub enum Formula {
    Atom(String),
    Not(Box<Formula>),
    And(Box<Formula>, Box<Formula>),
    Or(Box<Formula>, Box<Formula>),
    Implies(Box<Formula>, Box<Formula>),
    Iff(Box<Formula>, Box<Formula>),
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

// pub struct Parser {
//     tokens: Peekable<IntoIter<Token>>,
// }
