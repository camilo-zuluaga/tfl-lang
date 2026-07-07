use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_] => ExitCode::from(64),
        [_, path] => run_file(path),
        _ => {
            println!("Usage: tfl [file path]");
            ExitCode::from(64)
        }
    }
}

fn run_file(path: &str) -> ExitCode {
    match fs::read_to_string(path) {
        Ok(content) => {
            run(&content);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("tfl: cannot read '{path}': {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(source: &str) {
    for token in source.split_whitespace() {
        println!("{token}");
    }
}

/*
will need a struct after to properly give errors
*/
enum Token {
    LeftParen,
    RightParen,
    Atom(String),
    And,
    Or,
    Iff,
    Implies,
    Not,
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
        }
    }

    fn scan_tokens(self) -> Result<Vec<Token>, LexError> {
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
                    _ => return Err(LexError::Unexpected()),
                },
                // we should expect the <-> biconditional
                '<' => match (chars.next(), chars.next()) {
                    (Some('-'), Some('>')) => tokens.push(Token::Iff), 
                    _ => return Err(LexError::Unexpected())
                }
                _ => return Err(LexError::Unexpected())
            }
        }
        Ok(tokens)
    }
}

enum LexError {
    Unexpected(),
}
