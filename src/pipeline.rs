use std::fmt;

use crate::{
    lexer::{self, LexError},
    parser::{Formula, ParseError, Parser},
};

pub enum PipelineError {
    Lex(LexError),
    Parse(ParseError),
}

impl From<LexError> for PipelineError {
    fn from(value: LexError) -> Self {
        PipelineError::Lex(value)
    }
}

impl From<ParseError> for PipelineError {
    fn from(value: ParseError) -> Self {
        PipelineError::Parse(value)
    }
}

impl fmt::Display for PipelineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineError::Lex(e) => write!(f, "lex error: {e}"),
            PipelineError::Parse(e) => write!(f, "parser error: {e}"),
        }
    }
}

pub fn parse_source(source: &str) -> Result<Formula, PipelineError> {
    let tokens = lexer::tokenize(source)?;
    Ok(Parser::new(tokens).parse()?)
}
