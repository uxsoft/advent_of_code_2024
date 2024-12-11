use ilog::IntLog;
use std::{
    collections::HashMap,
    ops::{Add, Rem},
};

#[inline(always)]
fn is_even_digits(n: &usize) -> bool {
    n.log10().add(1).rem(2) == 0
}

fn split_number(n: &usize) -> (usize, usize) {
    let mut first = n.to_string();
    let second = first.split_off(first.len() / 2);

    (first.parse().unwrap(), second.parse().unwrap())
}

fn put_stone(cache: &mut HashMap<usize, usize>, value: usize, count: usize) {
    cache
        .entry(value)
        .and_modify(|e| *e += count)
        .or_insert(count);
}

pub fn process(input: &str) -> usize {
    let mut stones: HashMap<usize, usize> = super::parse(input).iter().map(|v| (*v, 1)).collect();

    for _gen in 0..75 {
        let mut new_stones = HashMap::new();

        for (value, count) in stones {
            match value {
                0 => put_stone(&mut new_stones, 1, count),
                even if is_even_digits(&even) => {
                    let (first, second) = split_number(&even);
                    put_stone(&mut new_stones, first, count);
                    put_stone(&mut new_stones, second, count);
                }
                other => put_stone(&mut new_stones, other * 2024, count),
            }
        }

        stones = new_stones;
    }

    stones.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_even_digits() {
        // assert_eq!(false, super::is_even_digits(&0));
        assert_eq!(false, super::is_even_digits(&1));
        assert_eq!(true, super::is_even_digits(&10));
        assert_eq!(true, super::is_even_digits(&11));
        assert_eq!(true, super::is_even_digits(&19));
        assert_eq!(false, super::is_even_digits(&100));
        assert_eq!(false, super::is_even_digits(&199));
        assert_eq!(true, super::is_even_digits(&1234567890));
        assert_eq!(false, super::is_even_digits(&123456789));
    }

    #[test]
    fn example() {
        let input = "125 17";

        let result = process(input);

        assert_eq!(65601038650482, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        assert_eq!(270673834779359, result);
    }
}
