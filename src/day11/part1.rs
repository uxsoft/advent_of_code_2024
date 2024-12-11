use ilog::IntLog;
use std::ops::{Add, Mul, Rem};

#[inline(always)]
fn is_even_digits(n: &usize) -> bool {
    n.log10().add(1).rem(2) == 0
}

fn split_number(n: &usize) -> (usize, usize) {
    let mut first = n.to_string();
    let second = first.split_off(first.len() / 2);

    (first.parse().unwrap(), second.parse().unwrap())
}

pub fn process(input: &str) -> usize {
    let mut stones = super::parse(input);

    for _gen in 0..25 {
        for index in 0..stones.len(){
            match stones[index] {
                0 => stones[index] = 1,
                even if is_even_digits(&even) => {
                    let (first, second) = split_number(&even);
                    stones[index] = first;
                    stones.push(second);
                }
                other => stones[index] = other.mul(2024),
            }
        }

        //println!("STONES: {}", stones.iter().join(" "));
    }

    stones.len()
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

        assert_eq!(55312, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        assert_eq!(228668, result);
    }
}
