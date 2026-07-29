use std::collections::HashSet;

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
