use std::collections::{HashMap, HashSet};

use crate::parser::Formula;

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
    let mut atoms: Vec<String> = set.into_iter().collect();
    atoms.sort();
    atoms
}


fn eval(formula: &Formula, assignment: &HashMap<&String, bool>) -> bool {
    match formula {
        Formula::Atom(name) => assignment[name],
        Formula::Not(inner) => !eval(inner, assignment),
        Formula::And(l, r) => eval(l, assignment) && eval(r, assignment),
        Formula::Or(l, r) => eval(l, assignment) || eval(r, assignment),
        Formula::Implies(l, r) => !eval(l, assignment) || eval(r, assignment),
        Formula::Iff(l, r) => eval(l, assignment) == eval(r, assignment),
    }
}
