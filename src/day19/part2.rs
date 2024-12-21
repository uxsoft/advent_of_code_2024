use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn is_possible<'a>(cache: &mut HashMap<&'a str, usize>, parts: &'a Vec<&'a str>, pattern: &'a str) -> usize {
    if let Some(cached_result) = cache.get(pattern) {
        return *cached_result;
    }

    let total = parts
        .into_iter()
        .map(|part| {
            if let Some(new_pattern) = pattern.strip_prefix(part) {
                is_possible(cache, parts, new_pattern)
            } else {
                0
            }
        })
        .sum();

    cache.insert(pattern, total);
    return total;
}

pub fn process(input: &str) -> usize {
    let (mut parts, patterns) = super::parse(input);
    parts.sort_by_key(|p| p.len());

    let mut cache = HashMap::new();
    cache.insert("", 1);

    let possible = patterns
        .iter()
        .map(|pat| is_possible(&mut cache, &parts, pat))
        .sum();

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
        assert_eq!(result, 16);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input);
        assert_eq!(result, 572248688842069);
    }
}
