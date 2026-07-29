use ansiterm::Style;
use std::collections::{HashMap, HashSet};

use crate::ast::Formula;

fn collect_atoms(formula: &Formula, atoms: &mut HashSet<String>) {
    match formula {
        Formula::Atom(name) => {
            atoms.insert(name.clone());
        }
        Formula::Not(inner) => collect_atoms(inner, atoms),
        Formula::And(l, r) => {
            collect_atoms(l, atoms);
            collect_atoms(r, atoms);
        }
        Formula::Or(l, r) => {
            collect_atoms(l, atoms);
            collect_atoms(r, atoms);
        }
        Formula::Implies(l, r) => {
            collect_atoms(l, atoms);
            collect_atoms(r, atoms);
        }
        Formula::Iff(l, r) => {
            collect_atoms(l, atoms);
            collect_atoms(r, atoms);
        }
    }
}

fn atoms_of(formula: &Formula) -> Vec<String> {
    let mut set = HashSet::new();
    collect_atoms(formula, &mut set);
    let mut v: Vec<String> = set.into_iter().collect();
    v.sort();
    v
}

fn assignment_for(i: usize, atoms: &[String]) -> HashMap<String, bool> {
    let mut assignment: HashMap<String, bool> = HashMap::new();
    for (j, atom) in atoms.iter().enumerate() {
        // read bit j of i: shift it down to the lowest position, then mask it
        // bit j is atom j's truth value in this row
        let val = (i >> j) & 1 == 0;
        assignment.insert(atom.clone(), val);
    }
    assignment
}

fn result_column(formula: &Formula) -> Vec<bool> {
    let atoms = atoms_of(formula);
    // 1 << n is the same as 2^n, left shift doubles the number and that is what we want
    // if we got 2 atoms, it means we will have 4 lines, 3 atoms will have 8 lines, and so on
    (0..(1 << atoms.len()))
        .map(|i| eval(&formula, &assignment_for(i, &atoms)))
        .collect()
}

fn is_tautology(column: &[bool]) -> bool {
    column.iter().all(|&b| b)
}

fn is_contradiction(column: &[bool]) -> bool {
    column.iter().all(|&b| !b)
}

fn check_semantic(b: &[bool]) {
    if is_tautology(b) {
        println!("{}", Style::new().bold().italic().paint("\ntautology."));
    } else if is_contradiction(b) {
        println!("{}", Style::new().bold().italic().paint("\ncontradiction."));
    } else {
        println!("{}", Style::new().bold().italic().paint("\ncontingent."));
    }
    println!();
}

pub fn truth_table(formula: &Formula) {
    let atoms = atoms_of(formula);
    let col = result_column(&formula);

    println!("");
    for atom in &atoms {
        print!("{atom} ");
    }
    println!("| {formula}");
    println!("{}", "-".repeat(formula.to_string().len()));

    for (i, res) in col.iter().enumerate() {
        let assignment = assignment_for(i, &atoms);

        for atom in &atoms {
            let v = assignment[atom];
            print!("{} ", if v { "T" } else { "F" });
        }

        println!(
            "| {}",
            if *res {
                Style::new().bold().paint("T")
            } else {
                Style::new().bold().paint("F")
            }
        );
    }

    check_semantic(&col);
}

fn eval(formula: &Formula, assignment: &HashMap<String, bool>) -> bool {
    match formula {
        Formula::Atom(name) => assignment[name],
        Formula::Not(inner) => !eval(inner, assignment),
        Formula::And(l, r) => eval(l, assignment) && eval(r, assignment),
        Formula::Or(l, r) => eval(l, assignment) || eval(r, assignment),
        Formula::Implies(l, r) => !eval(l, assignment) || eval(r, assignment),
        Formula::Iff(l, r) => eval(l, assignment) == eval(r, assignment),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn assign(s: &[(&str, bool)]) -> HashMap<String, bool> {
        s.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    #[test]
    fn atom_not_flips() {
        let f = Formula::not(Formula::atom("A"));
        assert_eq!(eval(&f, &assign(&vec![("A", true)])), false);
        assert_eq!(eval(&f, &assign(&vec![("A", false)])), true);
    }

    #[test]
    fn and_validation() {
        let f = Formula::and(Formula::atom("A"), Formula::atom("B"));
        assert_eq!(eval(&f, &assign(&vec![("A", true), ("B", false)])), false);
        assert_eq!(eval(&f, &assign(&vec![("A", true), ("B", true)])), true);
    }

    #[test]
    fn or_validaton() {
        let f = Formula::or(Formula::atom("A"), Formula::atom("B"));
        assert_eq!(eval(&f, &assign(&vec![("A", true), ("B", false)])), true);
        assert_eq!(eval(&f, &assign(&vec![("A", false), ("B", false)])), false);
    }

    #[test]
    fn implies_validation() {
        let f = Formula::implies(Formula::atom("A"), Formula::atom("B"));
        assert_eq!(eval(&f, &assign(&vec![("A", true), ("B", false)])), false);
        assert_eq!(eval(&f, &assign(&vec![("A", false), ("B", true)])), true);
    }

    #[test]
    fn iff_validation() {
        let f = Formula::implies(Formula::atom("A"), Formula::atom("B"));
        assert_eq!(eval(&f, &assign(&vec![("A", true), ("B", false)])), false);
        assert_eq!(eval(&f, &assign(&vec![("A", true), ("B", true)])), true);
        assert_eq!(eval(&f, &assign(&vec![("A", false), ("B", false)])), true);
    }
}
