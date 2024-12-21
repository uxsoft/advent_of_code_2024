use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

fn is_possible(parts: &HashSet<&str>, pattern: &str) -> bool {
    if pattern.len() == 0 {
        return true;
    }

    for part in parts {
        if let Some(new_pattern) = pattern.strip_prefix(part) {
            if is_possible(parts, new_pattern) {
                return true;
            }
        }
    }

    return false;
}

fn filter_atomic_parts<'a>(parts: &'a Vec<&'a str>) -> HashSet<&'a str> {
    let mut new_parts: HashSet<&str> = parts.into_iter().copied().collect();

    for pattern in parts {
        new_parts.remove(pattern);
        if !is_possible(&new_parts, pattern) {
            new_parts.insert(pattern);
        }
    }

    new_parts
}

pub fn process(input: &str) -> usize {
    let (parts, patterns) = super::parse(input);

    let filtered_parts = filter_atomic_parts(&parts);
    println!("REDUCED PARTS {} => {}", parts.len(), filtered_parts.len());

    let possible = patterns
        .iter()
        .filter(|pat| is_possible(&filtered_parts, pat))
        .count();

    possible
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn example() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

        let result = process(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input);
        assert_eq!(result, 298);
    }
}
