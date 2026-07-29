use reedline::{DefaultPrompt, Reedline, Signal};
use std::{env, fs, process::ExitCode};
use {
    crossterm::event::{KeyCode, KeyModifiers},
    reedline::{EditCommand, Emacs, ReedlineEvent, default_emacs_keybindings},
};

use crate::parser::Parser;

mod eval;
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
    // just getting it done
    let mut kb = default_emacs_keybindings();
    for (ch, sym) in [('a', "∧"), ('o', "∨"), ('n', "¬"), ('i', "→"), ('b', "↔")] {
        kb.add_binding(
            KeyModifiers::CONTROL,
            KeyCode::Char(ch),
            ReedlineEvent::Edit(vec![EditCommand::InsertString(sym.to_string())]),
        );
    }
    let edit_mode = Box::new(Emacs::new(kb));
    let mut line_editor = Reedline::create().with_edit_mode(edit_mode);
    let prompt = DefaultPrompt::default();

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                println!("We processed: {}", buffer);
            }
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            x => {
                println!("Event: {:?}", x);
            }
        }
    }
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
