use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

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
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return ExitCode::FAILURE,
    };
    run(content);
    ExitCode::SUCCESS
}

fn run(source: String) {
    let tokens: Vec<_> = source.split(" ").collect();
    for token in tokens {
        println!("{}", token);
    }
}
