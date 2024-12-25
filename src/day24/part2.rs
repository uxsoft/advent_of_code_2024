use std::{
    collections::{HashMap, HashSet, VecDeque},
    mem::replace,
};

use chumsky::container::Container;
use indicatif::ProgressIterator;
use itertools::Itertools;

use super::{parse, Op};

fn bools_to_num<'a>(bools: impl Iterator<Item = &'a bool>) -> usize {
    bools.fold(0, |v, b| (v << 1) + (*b as usize))
}

fn num_to_bools(num: usize) -> Vec<bool> {
    let mut num = num;
    let mut bools = Vec::new();

    while num > 0 {
        bools.push((num & 1) != 0);
        num >>= 1;
    }

    bools
}

fn bool_vec_to_string<'a>(bools: impl Iterator<Item = &'a bool>) -> String {
    bools.map(|b| *b as u8).join("")
}

fn eval_statements(
    vars: &Vec<(String, bool)>,
    statements: &Vec<super::Statement>,
) -> HashMap<String, bool> {
    let mut vars: HashMap<_, _> = vars.into_iter().cloned().collect();

    let mut statements: VecDeque<_> = statements.into_iter().cloned().collect();

    while let Some((a, op, b, to)) = statements.pop_front() {
        if vars.contains_key(&a) && vars.contains_key(&b) {
            vars.insert(to, op.eval(vars[&a], vars[&b]));
        } else {
            statements.push_back((a, op, b, to));
        }
    }

    vars
}

fn eval_statements_w_replacements(
    vars: &Vec<(String, bool)>,
    statements: &Vec<super::Statement>,
    replacements: HashMap<String, String>,
) -> HashMap<String, bool> {
    let mut vars: HashMap<String, bool> = vars.into_iter().cloned().collect();

    let mut statements: VecDeque<super::Statement> = statements.into_iter().cloned().collect();

    let mut i = 0;
    while let Some((a, op, b, to)) = statements.pop_front() {
        i += 1;
        if i > 10000 {
            return vars;
        }
        if vars.contains_key(&a) && vars.contains_key(&b) {
            let to = replacements.get(&to).unwrap_or(&to);
            vars.insert(to.clone(), op.eval(vars[&a], vars[&b]));
        } else {
            statements.push_back((a, op, b, to));
        }
    }

    vars
}

fn filter_vars(vars: &HashMap<String, bool>, prefix: char) -> Vec<bool> {
    vars.iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .sorted_by_key(|(k, _)| *k)
        .map(|(_, v)| *v)
        .rev()
        .collect()
}

fn eval_statements_rec(
    vars: &HashMap<String, bool>,
    statements: &Vec<super::Statement>,
    replacements: HashMap<String, String>,
) -> Option<Vec<(String, bool)>> {
    let mut values = HashMap::new();
    let statement_map = statements
        .into_iter()
        .fold(HashMap::new(), |mut acc, (a, op, b, to)| {
            let to = replacements.get(to).unwrap_or(to).clone();
            acc.insert(to.clone(), (a.clone(), op.clone(), b.clone(), to.clone()));
            acc
        });

    let mut temporary_mark = HashSet::new();

    fn eval(
        node: String,
        vars: &HashMap<String, bool>,
        values: &mut HashMap<String, bool>,
        statement_map: &HashMap<String, super::Statement>,
        temporary_mark: &mut HashSet<String>,
    ) -> Option<bool> {
        if let Some(v) = vars.get(&node) {
            return Some(*v);
        }
        if temporary_mark.contains(&node) {
            return None;
        }

        temporary_mark.insert(node.clone());

        let (a, op, b, to) = statement_map[&node].clone();
        let a_opt = values
            .get(&a)
            .map(|v| *v)
            .or_else(|| eval(a, vars, values, statement_map, temporary_mark));

        let b_opt = values
            .get(&b)
            .map(|v| *v)
            .or_else(|| eval(b, vars, values, statement_map, temporary_mark));

        if let (Some(a_value), Some(b_value)) = (a_opt, b_opt) {
            let result = op.eval(a_value, b_value);
            values.insert(to, result);
            return Some(result);
        } else {
            return None;
        }
    }

    let results: Vec<_> = statement_map
        .keys()
        .filter(|n| n.starts_with('z'))
        .sorted()
        .cloned()
        .map(|n| {
            (
                n.clone(),
                eval(n, vars, &mut values, &statement_map, &mut temporary_mark),
            )
        })
        .collect();

    if results.iter().any(|(_, v)| v.is_none()) {
        return None;
    } else {
        return Some(results.into_iter().map(|(k, v)| (k, v.unwrap())).collect());
    }
}

pub fn process(input: &str) -> String {
    let (vars, statements) = parse(input);

    let mut faulty = Vec::new();

    for (a, op, b, to) in &statements {
        // 1. If there is output to a z-wire, the operator should always be XOR (unless
        // it is the final bit). If not -> faulty.
        if to.starts_with('z') && op != &Op::XOR && !to.starts_with("z45") {
            faulty.push(to.clone());
        }

        // 2. If the output is not a z-wire and the inputs are not x and y, the operator
        // should always be AND or OR. If not -> faulty.
        if !to.starts_with('z')
            && !a.starts_with('x')
            && !a.starts_with('y')
            && !b.starts_with('x')
            && !b.starts_with('y')
            && op == &Op::XOR
        {
            faulty.push(to.clone());
        }

        // 3. If the inputs are x and y (but not the first bit) and the operator is XOR,
        // the output wire should be the input of another XOR-gate. If not -> faulty.
        // 4. If the inputs are x and y (but not the first bit) and the operator is AND,
        // the output wire should be the input of an OR-gate. If not -> faulty.
        if (a.starts_with('x') || a.starts_with('y'))
            && (b.starts_with('x') || b.starts_with('y'))
            && !(a.ends_with("00") || b.ends_with("00"))
        {
            let expected_next_op = if op == &Op::XOR { Op::XOR } else { Op::OR };

            let has_expected_consumer = statements
                .iter()
                .any(|(a2, op2, b2, to2)| op2 == &expected_next_op && (to == a2 || to == b2));

            if !has_expected_consumer {
                faulty.push(to.clone());
            }
        }
    }

    let result = faulty.into_iter().unique().sorted().join(",");
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input);
        assert_eq!(result, "gbs,hwq,thm,wrm,wss,z08,z22,z29".to_string());
    }
}
