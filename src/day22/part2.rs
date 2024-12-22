use std::collections::HashMap;

use itertools::Itertools;

type Seq = (i8, i8, i8, i8);

fn compute(initial: usize, steps: usize) -> HashMap<Seq, i8> {
    let mut iterator = super::PseudoRandomIterator::new(initial);

    let mut map = HashMap::new();

    iterator
        .take(steps - 1)
        .map(|i| (i % 10) as i8) // ones digit
        .tuple_windows()
        .map(|(a, b)| (b - a, b)) // changes
        .tuple_windows()
        .for_each(|((a, _), (b, _), (c, _), (d, v))| {
            map.entry((a, b, c, d)).or_insert(v);
        });

    map
}

fn merge(maps: Vec<HashMap<Seq, i8>>) -> HashMap<Seq, i64> {
    maps.into_iter().fold(HashMap::new(), |mut acc, item| {
        for (k, v) in item {
            acc.entry(k)
                .and_modify(|p| *p += v as i64)
                .or_insert(v as i64);
        }
        acc
    })
}

pub fn process(input: &str) -> i64 {
    let numbers = super::parse(input);
    let maps = numbers.into_iter().map(|i| compute(i, 2000)).collect();
    let supermap = merge(maps);

    let total = supermap.values().max().unwrap();

    *total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial() {
        let map = compute(123, 10);
        let seq = (-1, -1, 0, 2);
        dbg!(&map);
        assert_eq!(map[&seq], 6);
    }

    #[test]
    fn example() {
        let seq = (-2, 1, -1, 3);
        let cases = vec![(1, 7), (2, 7), (3, 0), (2024, 9)];
        for (initial, expected) in cases {
            let map = compute(initial, 2000);
            assert_eq!(*map.get(&seq).unwrap_or(&0), expected);
        }
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let output = process(input);
        assert_eq!(output, 1667);
    }
}
