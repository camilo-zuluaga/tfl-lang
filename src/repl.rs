use reedline::{
    EditCommand, Emacs, KeyCode, KeyModifiers, Keybindings, Prompt, PromptEditMode,
    PromptHistorySearch, Reedline, ReedlineEvent, Signal, default_emacs_keybindings,
};
use std::{borrow::Cow, io};

use crate::{eval, pipeline::parse_source};

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
    let mut line_editor = Reedline::create().with_edit_mode(edit_mode);
    let prompt = SimpleColoredPrompt;

    loop {
        let sig = line_editor.read_line(&prompt)?;
        match sig {
            Signal::Success(buffer) => {
                run_formula(&buffer);
            }
            Signal::CtrlD | Signal::CtrlC => {
                println!("\nBye!");
                break Ok(());
            }
            _ => {}
        }
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
