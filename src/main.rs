use crate::pipeline::PipelineError;

use std::{env, fs, process::ExitCode};

mod eval;
mod lexer;
mod parser;
mod pipeline;
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
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("tfl: cannot read '{path}': {e}");
            return ExitCode::FAILURE;
        }
    };

    match run(&content) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("tfl: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run_prompt() -> ExitCode {
    match repl::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("tfl: repl error {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(source: &str) -> Result<(), PipelineError> {
    let f = pipeline::parse_source(source)?;
    eval::truth_table(&f);
    Ok(())
}
