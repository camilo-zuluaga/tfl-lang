use crate::parser::Parser;

use std::{env, fs, process::ExitCode};

mod eval;
mod lexer;
mod parser;
mod repl;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_] => run_prompt(),
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

fn run_prompt() -> ExitCode {
    let _ = repl::run();
    ExitCode::SUCCESS
}

fn run(source: &str) {
    match lexer::tokenize(source) {
        Ok(tokens) => match Parser::new(tokens).parse() {
            Ok(formula) => eval::truth_table(&formula),
            Err(e) => eprintln!("[parser error] {e}"),
        },
        Err(e) => eprintln!("lex error {e:?}"),
    }
}
