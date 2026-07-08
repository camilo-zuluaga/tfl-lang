/*
will need a struct after to properly give errors
*/
#[derive(Debug)]
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

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(self) -> Result<Vec<Token>, LexError> {
        let mut tokens = self.tokens;
        let mut chars = self.source.chars().peekable();

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
                    _ => return Err(LexError::Unexpected(c)),
                },
                // we should expect the <-> biconditional
                '<' => match (chars.next(), chars.next()) {
                    (Some('-'), Some('>')) => tokens.push(Token::Iff),
                    _ => return Err(LexError::Unexpected(c)),
                },
                // TODO: Implement atomic sentences
                _ => return Err(LexError::Unexpected(c)),
            }
        }
        Ok(tokens)
    }
}

#[derive(Debug)]
pub enum LexError {
    Unexpected(char),
}
