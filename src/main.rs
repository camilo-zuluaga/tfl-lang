use std::{env, fs, io::{self, Write}, process::ExitCode};

use crate::parser::Parser;

mod lexer;
mod parser;

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
    let mut line = String::new();
    loop {
        print!("> ");
        // classic buffering problem
        io::stdout().flush().expect("flush failed");
        line.clear();
        io::stdin().read_line(&mut line).expect("failed to read");
        run(&line);
    }
}

fn run(source: &str) {
    match lexer::tokenize(source) {
        Ok(tokens) => match Parser::new(tokens).parse() {
            Ok(formula) => println!("{formula}"),
            Err(e) => eprintln!("[parser error] {e}"),
        },
        Err(e) => eprintln!("lex error {e:?}"),
    }
}
