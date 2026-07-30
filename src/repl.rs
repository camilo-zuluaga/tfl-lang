use ansiterm::Style;
use nu_ansi_term::Color;
use reedline::{
    EditCommand, Emacs, ExampleHighlighter, KeyCode, KeyModifiers, Keybindings, Prompt,
    PromptEditMode, PromptHistorySearch, Reedline, ReedlineEvent, Signal,
    default_emacs_keybindings,
};
use std::{borrow::Cow, io};

use crate::{
    ast::Formula,
    eval::{self},
    pipeline::{self, parse_source},
};

#[derive(Clone)]
pub struct SimpleColoredPrompt;

impl Prompt for SimpleColoredPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        Cow::Borrowed("\x1b[35mtfl> \x1b[0m")
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_indicator(&self, _edit_mode: PromptEditMode) -> Cow<'_, str> {
        Cow::Borrowed("")
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        Cow::Borrowed("\x1b[32m::: \x1b[0m")
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<'_, str> {
        Cow::Owned(format!("(reverse-search: {}) ", history_search.term))
    }
}

fn commands() -> ExampleHighlighter {
    let mut commands = ExampleHighlighter::new(vec![
        ":taut".into(),
        ":contra".into(),
        ":entail".into(),
        ":help".into(),
    ]);
    commands.change_colors(Color::Blue, Color::Default, Color::Default);
    commands
}

fn custom_keybindings() -> Keybindings {
    let mut kb = default_emacs_keybindings();
    for (ch, sym) in [('a', "∧"), ('o', "∨"), ('n', "¬"), ('i', "→"), ('b', "↔")] {
        kb.add_binding(
            KeyModifiers::ALT,
            KeyCode::Char(ch),
            ReedlineEvent::Edit(vec![EditCommand::InsertString(sym.to_string())]),
        );
    }
    kb
}

pub fn run() -> io::Result<()> {
    println!("TFL REPL.\nAbort with Ctrl-C or Ctrl-D");

    let edit_mode = Box::new(Emacs::new(custom_keybindings()));
    let cmds = Box::new(commands());
    let mut line_editor = Reedline::create()
        .with_edit_mode(edit_mode)
        .with_highlighter(cmds);
    let prompt = SimpleColoredPrompt;

    loop {
        let sig = line_editor.read_line(&prompt)?;
        match sig {
            Signal::Success(buffer) => {
                process_line(&buffer);
            }
            Signal::CtrlD | Signal::CtrlC => {
                println!("\nBye!");
                break Ok(());
            }
            _ => {}
        }
    }
}

fn process_line(line: &str) {
    if line.is_empty() {
        return;
    }
    if let Some(cmd) = line.strip_prefix(":") {
        run_command(cmd);
    } else {
        run_formula(line);
    }
}

fn run_formula(line: &str) {
    match parse_source(line) {
        Err(e) => eprintln!(" {e}"),
        Ok(formula) => {
            eval::truth_table(&formula);
        }
    }
}

fn run_command(cmd: &str) {
    let (word, rest) = match cmd.split_once(char::is_whitespace) {
        Some((w, r)) => (w, r.trim()),
        None => (cmd, ""),
    };

    match word {
        "help" => print_help(),
        "taut" => check_taut(rest),
        "contra" => check_contra(rest),
        "entail" => check_entail(rest),
        other => println!(" unknown command: {other}"),
    }
}

fn check_entail(input: &str) {
    let (prem, concl) = match input.split_once("|=") {
        Some((p, c)) => (p.trim(), c.trim()),
        None => {
            eprintln!(" usage: :entails p1, p2 |= conclusion");
            return;
        }
    };

    let mut premises: Vec<Formula> = Vec::new();
    for s in prem.split(',') {
        let s = s.trim();
        if s.is_empty() {
            continue;
        }
        match pipeline::parse_source(s) {
            Ok(f) => premises.push(f),
            Err(e) => {
                eprintln!(" premise error: {e}");
            }
        }
    }

    let conclusion = match pipeline::parse_source(concl) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(" conclusion error: {e}");
            return;
        }
    };

    println!();
    match eval::entails(&premises, &conclusion) {
        None => println!("{}", Style::new().bold().italic().paint(" entails.")),
        Some(i) => {
            let atoms = eval::atoms_of_all(&premises, &conclusion);
            let asn = eval::assignment_for(i, &atoms);
            let row: Vec<String> = atoms
                .iter()
                .map(|a| format!("{a}={}", if asn[a] { "T" } else { "F" }))
                .collect();
            println!(
                " {} {}",
                Style::new().bold().italic().paint(" does not entail: "),
                row.join(", ")
            );
        }
    }
    println!();
}

fn check_contra(f: &str) {
    if f.is_empty() {
        println!(" usage :contra <formula>");
        return;
    }

    match pipeline::parse_source(f) {
        Err(e) => eprintln!(" {e}"),
        Ok(formula) => {
            let col = eval::result_column(&formula);
            println!();
            if eval::is_contradiction(&col) {
                println!(
                    " {formula} {}",
                    Style::new().bold().italic().paint("is a contradiction")
                );
            } else {
                let atoms = eval::atoms_of(&formula);
                let i = col.iter().position(|&b| b).unwrap();
                let asn = eval::assignment_for(i, &atoms);
                let row: Vec<String> = atoms
                    .iter()
                    .map(|a| format!("{a}={}", if asn[a] { "T" } else { "F" }))
                    .collect();
                println!(
                    " {} {}",
                    Style::new()
                        .bold()
                        .italic()
                        .paint(" not a contradiction, true when "),
                    row.join(", ")
                );
            }
            println!();
        }
    }
}

fn check_taut(f: &str) {
    if f.is_empty() {
        println!(" usage: :taut <formula>");
        return;
    }

    match pipeline::parse_source(f) {
        Err(e) => eprintln!(" {e}"),
        Ok(formula) => {
            let col = eval::result_column(&formula);
            println!();
            if eval::is_tautology(&col) {
                println!(
                    " {formula} {}",
                    Style::new().bold().italic().paint("is a tautology")
                )
            } else {
                let atoms = eval::atoms_of(&formula);
                let i = col.iter().position(|&b| !b).unwrap(); // this unwrap is `safe` since is not
                // a tautology, it means there is at least one False
                let asn = eval::assignment_for(i, &atoms);
                let row: Vec<String> = atoms
                    .iter()
                    .map(|a| format!("{a}={}", if asn[a] { "T" } else { "F" }))
                    .collect();
                println!(
                    " {} {}",
                    Style::new()
                        .bold()
                        .italic()
                        .paint(" not a tautology, false when "),
                    row.join(", ")
                );
            }
            println!();
        }
    }
}

fn print_help() {
    println!(
        "\
tfl — truth-functional logic

  Type a formula to see its truth table and classification:
    (P -> Q) & ~R

  Connectives (ASCII accepted, printed as symbols):
    ~ !         negation      ¬
    &           conjunction   ∧
    |           disjunction   ∨
    ->          conditional   →
    <->         biconditional ↔
    atoms: uppercase + digits (P, Q, S1)   comments: -- to end of line

  Commands:
    :taut     <formula>              is it a tautology?
    :contra   <formula>              is it a contradiction?
    :entails  <p1>, <p2> |= <concl>  do the premises entail the conclusion?
    :help                            this message
    :quit                            exit  (or Ctrl-D)

  Symbol entry: Alt+a ∧   Alt+o ∨   Alt+n ¬   Alt+i →   Alt+b ↔
"
    );
}
