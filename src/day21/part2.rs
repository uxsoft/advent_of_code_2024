use std::{collections::HashMap, iter, ops::Neg};

use itertools::Itertools;

use super::keypad::KeyPad;

fn chain(
    cache: &mut HashMap<(String, usize), usize>,
    input: String,
    depth: usize,
    is_numeric: bool,
) -> usize {
    if let Some(cached_result) = cache.get(&(input.clone(), depth)) {
        return *cached_result;
    }

    let paths = if is_numeric {
        KeyPad::numpad().shortest_paths()
    } else {
        KeyPad::arrows().shortest_paths()
    };

    let total = format!("A{}", &input)
        .chars()
        .tuple_windows()
        .map(|(a, b)| {
            let shortest_paths = paths.get(&(a, b)).unwrap();
            if depth == 0 {
                shortest_paths[0].len()
            } else {
                shortest_paths
                    .iter()
                    .map(|p| chain(cache, p.to_string(), depth - 1, false))
                    .min()
                    .unwrap()
            }
        })
        .sum();

    cache.insert((input, depth), total);

    total
}

pub fn process(input: &str) -> usize {
    let values: Vec<usize> = input
        .lines()
        .map(|line| line.strip_suffix("A").unwrap().parse().unwrap())
        .collect();

    let mut cache = HashMap::new();

    let total = input
        .lines()
        .map(|s| s.to_string())
        .zip(values)
        .map(|(s, v)| v * chain(&mut cache, s, 25, true))
        .sum();

    total
}

#[cfg(test)]
mod tests {
    use chumsky::input;
    use itertools::Itertools;

    use super::*;

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let output = process(input);
        assert_eq!(output, 157055032722640);
    }
}
