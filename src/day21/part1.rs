use std::{collections::HashMap, iter, ops::Neg};

use itertools::Itertools;

use super::keypad::KeyPad;

fn numeric_shortest_paths() -> HashMap<(char, char), Vec<String>> {
    let np = KeyPad::numpad();

    let keys = "0123456789A";

    let map = keys
        .chars()
        .cartesian_product(keys.chars())
        .map(|(a, b)| {
            let ap = np.locate(a);
            let bp = np.locate(b);

            let delta_x = bp.x - ap.x;
            let delta_y = bp.y - ap.y;

            let vertical = if delta_y.is_negative() {
                "^".repeat(delta_y.neg() as usize)
            } else {
                "v".repeat(delta_y as usize)
            };

            let horizontal = if delta_x.is_negative() {
                "<".repeat(delta_x.neg() as usize)
            } else {
                ">".repeat(delta_x as usize)
            };

            let mut paths = Vec::new();
            if (ap.x == 0 && bp.y == 3) || delta_x == 0 || delta_y == 0 {
                paths.push(format!("{}{}A", horizontal, vertical));
            } else if bp.x == 0 && ap.y == 3 {
                paths.push(format!("{}{}A", vertical, horizontal));
            } else {
                paths.push(format!("{}{}A", vertical, horizontal));
                paths.push(format!("{}{}A", horizontal, vertical));
            }

            ((a, b), paths)
        })
        .collect();

    map
}

fn arrows_shortest_paths() -> HashMap<(char, char), Vec<String>> {
    let np = KeyPad::arrows();

    let keys = "<^v>A";

    let map = keys
        .chars()
        .cartesian_product(keys.chars())
        .map(|(a, b)| {
            let ap = np.locate(a);
            let bp = np.locate(b);

            let delta_x = bp.x - ap.x;
            let delta_y = bp.y - ap.y;

            let vertical = if delta_y.is_negative() {
                "^".repeat(delta_y.neg() as usize)
            } else {
                "v".repeat(delta_y as usize)
            };

            let horizontal = if delta_x.is_negative() {
                "<".repeat(delta_x.neg() as usize)
            } else {
                ">".repeat(delta_x as usize)
            };

            let mut paths = Vec::new();
            if (ap.x == 0 && bp.y == 0) || delta_x == 0 || delta_y == 0 {
                paths.push(format!("{}{}A", horizontal, vertical));
            } else if bp.x == 0 && ap.y == 0 {
                paths.push(format!("{}{}A", vertical, horizontal));
            } else {
                paths.push(format!("{}{}A", vertical, horizontal));
                paths.push(format!("{}{}A", horizontal, vertical));
            }

            ((a, b), paths)
        })
        .collect();

    map
}

fn chain(input: String, depth: usize, is_numeric: bool) -> usize {
    let paths = if is_numeric {
        numeric_shortest_paths()
    } else {
        arrows_shortest_paths()
    };

    let total = format!("A{}", input)
        .chars()
        .tuple_windows()
        .map(|(a, b)| {
            let shortest_paths = paths.get(&(a, b)).unwrap();
            if depth == 0 {
                shortest_paths[0].len()
            } else {
                shortest_paths
                    .iter()
                    .map(|p| chain(p.to_string(), depth - 1, false))
                    .min()
                    .unwrap()
            }
        })
        .sum();

    total
}

pub fn process(input: &str) -> usize {
    let values: Vec<usize> = input
        .lines()
        .map(|line| line.strip_suffix("A").unwrap().parse().unwrap())
        .collect();

    let total = input
        .lines()
        .map(|s| s.to_string())
        .zip(values)
        .map(|(s, v)| v * chain(s, 2, true))
        .sum();

    total
}

#[cfg(test)]
mod tests {
    use chumsky::input;
    use itertools::Itertools;

    use super::*;

    #[test]
    fn level1() {
        let input = "029A";
        let expected = "<A^A>^^AvvvA";
        let result = chain(input.into(), 0, true);
        assert_eq!(result, expected.len());
    }

    #[test]
    fn level2() {
        let input = "029A";
        let expected = "v<<A>>^A<A>AvA<^AA>A<vAAA>^A";
        let result = chain(input.into(), 1, true);
        assert_eq!(result, expected.len());
    }

    #[test]
    fn level3() {
        let inputs = vec!["029A", "980A", "179A", "456A", "379A"];
        let expecteds = vec![
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
            "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A",
            "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
        ];

        for (input, expected) in inputs.into_iter().zip(expecteds) {
            let output = chain(input.into(), 2, true);
            assert_eq!(output, expected.len());
        }
    }

    #[test]
    fn example() {
        let input = "029A
980A
179A
456A
379A";
        let output = process(input);
        assert_eq!(output, 126384);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let output = process(input);
        assert_eq!(output, 0);
    }
}
