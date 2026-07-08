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

pub fn tokenize(source: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' => {}
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '~' | '!' | '¬' => tokens.push(Token::Not),
            '&' | '∧' => tokens.push(Token::And),
            '|' | '∨' => tokens.push(Token::Or),
            '→' => tokens.push(Token::Implies),
            '↔' => tokens.push(Token::Iff),
            // i guess we should also accept <-> or ->
            '-' => match chars.next() {
                Some('>') => tokens.push(Token::Implies),
                _ => {
                    return Err(LexError::Incomplete {
                        expected: "-> after '-",
                        found: Some(c),
                    });
                }
            },
            // we should expect the <-> biconditional
            '<' => match (chars.next(), chars.next()) {
                (Some('-'), Some('>')) => tokens.push(Token::Iff),
                _ => {
                    return Err(LexError::Incomplete {
                        expected: "<-> after '<-",
                        found: Some(c),
                    });
                }
            },
            // if it is uppercase it must be a symbolization key
            c if c.is_ascii_uppercase() => {
                let mut name = String::from(c);
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        name.push(d);
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Atom(name));
            }
            _ => return Err(LexError::Unexpected(c)),
        }
    }
    Ok(tokens)
}

#[derive(Debug)]
pub enum LexError {
    Unexpected(char),
    Incomplete {
        expected: &'static str,
        found: Option<char>,
    },
}

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
}
