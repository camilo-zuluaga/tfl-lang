use std::{fmt, iter::Peekable, str::Chars};

/*
will need a struct after to properly give errors
*/
#[derive(PartialEq, Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    Atom(String),
    And,
    Or,
    Iff,
    Implies,
    Not,
}

type Cursor<'a> = Peekable<Chars<'a>>;

fn skip_comment(chars: &mut Cursor) {
    while chars.peek().is_some_and(|&d| d == '\n') {
        chars.next();
    }
}

fn lex_atom(first: char, chars: &mut Cursor) -> Token {
    // an atomic sentence must be a capital letter, optionally followed by digits
    let mut name = String::from(first);
    while let Some(&d) = chars.peek() {
        if !d.is_ascii_digit() {
            break;
        }
        name.push(d);
        chars.next();
    }
    Token::Atom(name)
}

fn expect(chars: &mut Cursor, want: char, context: &'static str) -> Result<(), LexError> {
    match chars.next() {
        Some(d) if d == want => Ok(()),
        found => Err(LexError::Incomplete {
            expected: context,
            found,
        }),
    }
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            c if c.is_whitespace() => {}
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '~' | '!' | '¬' => tokens.push(Token::Not),
            '&' | '∧' => tokens.push(Token::And),
            '|' | '∨' => tokens.push(Token::Or),
            '→' => tokens.push(Token::Implies),
            '↔' => tokens.push(Token::Iff),

            // bi conditionals, and conditionals can also be represented in the form
            // <-> or ->
            '-' => match chars.next() {
                Some('>') => tokens.push(Token::Implies),
                Some('-') => skip_comment(&mut chars),
                found => {
                    return Err(LexError::Incomplete {
                        expected: "'>' after '-",
                        found,
                    });
                }
            },
            '<' => {
                expect(&mut chars, '-', "'-' after '<'")?;
                expect(&mut chars, '>', "'>' after '<-'")?;
                tokens.push(Token::Iff);
            },

            c if c.is_ascii_uppercase() => tokens.push(lex_atom(c, &mut chars)),
            other => return Err(LexError::Unexpected(other)),
        }
    }
    Ok(tokens)
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    Unexpected(char),
    Incomplete {
        expected: &'static str,
        found: Option<char>,
    },
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::Unexpected(c) => write!(f, "unexpected character '{c}'"),
            LexError::Incomplete {
                expected,
                found: Some(c),
            } => write!(f, "expected {expected}, found '{c}'"),
            LexError::Incomplete {
                expected,
                found: None,
            } => write!(f, "expected {expected}, but input ended"),
        }
    }
}

impl std::error::Error for LexError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_unicode() {
        let t = tokenize("(P → Q ∧ ¬R)").unwrap();
        assert_eq!(
            t,
            vec![
                Token::LeftParen,
                Token::Atom("P".into()),
                Token::Implies,
                Token::Atom("Q".into()),
                Token::And,
                Token::Not,
                Token::Atom("R".into()),
                Token::RightParen
            ]
        );
    }

    #[test]
    fn error_when_incomplete_biconditional() {
        let t = tokenize("(P <- Q)");
        assert_eq!(
            t,
            Err(LexError::Incomplete {
                expected: "'>' after '<-'",
                found: Some(' ')
            })
        );
    }
}
