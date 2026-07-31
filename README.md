# tfl

*A truth-functional logic interpreter built from a textbook.*

![Made with VHS](https://vhs.charm.sh/vhs-3mtyBunNN3Iff2vqiWucsO.gif)

## What this is

`tfl` is a complete interpreter for the language of truth-functional logic (TFL) as
defined in [*forall x: Calgary*](https://forallx.openlogicproject.org/), an open
introduction to formal logic. It lexes, parses, and evaluates TFL sentences, prints
complete truth tables, and decides the semantic questions of the logic: whether a
sentence is a tautology, a contradiction, or contingent; whether two sentences are
equivalent; and whether a set of premises entails a conclusion.

## Why build this

I built this to learn Rust. I wanted a project with enough real structure to
force me through the hard parts, ownership, recursive data types, error
handling, but small enough that I could actually finish it. An interpreter for
propositional logic fit: I was already reading *forall x: Calgary* for its own
sake, and a language defined in a textbook seemed like a clean thing to build.

## The language

The grammar is the book's, implemented strictly: every binary connective carries its
own pair of parentheses, so every well-formed sentence has exactly one reading.
Ambiguity is not resolved by precedence conventions.

Input is liberal; output is canonical. The lexer accepts both the book's symbols and
keyboard-friendly ASCII, and the printer always emits the book's notation:

| Connective    | Canonical | ASCII accepted   |
|---------------|-----------|------------------|
| negation      | ¬         | `~`, `!`         |
| conjunction   | ∧         | `&`         |
| disjunction   | ∨         | `\|`       |
| conditional   | →         | `->`             |
| biconditional | ↔         | `<->`            |

Atoms are an uppercase letter with optional digits (`P`, `Q`, `S1`, `E2`), matching
the book's notation. Comments run from `--` to end of line.

## Installation

`tfl` is written in Rust. You'll need a Rust toolchain

### From source

```sh
git clone https://github.com/camilo-zuluaga/tfl
cd tfl
cargo install --path .
```
Once it finishes, `tfl` is available from anywhere:

```sh
$ tfl                    # start the interactive REPL
$ tfl argument.tfl       # evaluate a file
```

### Running without installing

If you'd rather not install it system-wide, you can run it straight from the
cloned repository:

```sh
cargo run                   # REPL
cargo run -- argument.tfl   # file mode
```

## Usage

Interactive:

```
$ tfl
tfl> (P | ~P)
     (P ∨ ¬P)
     ...
     tautology.

tfl> :taut ((P -> Q) <-> (~Q -> ~P))
     tautology.

tfl> :taut (P -> Q)
     not a tautology, false when P=T, Q=F

tfl> :entails (~L -> (J | L)), ~L |= J
     entails.

tfl> :entails (P -> Q), Q |= P
     does not entail: P=F, Q=T
```

The `:entails` command checks semantic entailment (⊨) exactly as chapter 12 defines
it: the entailment holds if and only if no valuation makes every premise true and the
conclusion false.

From a file:

```
$ tfl argument.tfl
```

Symbol entry in the REPL: type ASCII and let the echo canonicalize it, use
Alt-chords (`Alt+a` ∧, `Alt+o` ∨, `Alt+n` ¬, `Alt+i` →, `Alt+b` ↔).

## Architecture

Chapters used.

| *forall x: Calgary*                        | 
|--------------------------------------------|
| Ch. 5  Connectives                        | 
| Ch. 6  Sentences of TFL                   | 
| Ch. 7  Ambiguity                          | 
| Ch. 9–10  Characteristic truth tables     | 
| Ch. 11  Complete truth tables             | 
| Ch. 12  Semantic concepts                 | 


## Acknowledgment

The language implemented here is defined in *forall x: Calgary — An Introduction to
Formal Logic*, by P. D. Magnus, Tim Button, and others, remixed and expanded by Aaron
Thomas-Bolduc and Richard Zach.
