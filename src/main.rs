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
