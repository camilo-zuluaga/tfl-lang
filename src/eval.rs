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
    set.into_iter().collect()
}

pub fn truth_table(formula: &Formula) {
    let atoms = atoms_of(formula);
    let n = atoms.len();

    // 1 << n is the same as 2^n, left shift doubles the number and that is what we want
    // if we got 2 atoms, it means we will have 4 lines, 3 atoms will have 8 lines, and so on
    for i in 0..(1 << n) {
        let mut assignment: HashMap<String, bool> = HashMap::new();
        for (j, atom) in atoms.iter().enumerate() {
            // read bit j of i: shift it down to the lowest position, then mask it
            // bit j is atom j's truth value in this row
            let val = (i >> j) & 1 == 0;
            assignment.insert(atom.clone(), val);
        }

        let res = eval(formula, &assignment);
        println!("result: {res}");
        println!("");
    }
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
}
