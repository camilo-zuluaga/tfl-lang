use std::{env, fs, process::ExitCode};

mod lexer;

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
    match lexer::tokenize(source) {
        Ok(tokens) => println!("{tokens:?}"),
        Err(e) => eprintln!("lex error {e:?}"),
    }
}
