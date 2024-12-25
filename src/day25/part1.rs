use itertools::Itertools;

use super::parse;

fn fits(lock: &super::Lock, key: &super::Key) -> bool {
    (0..5).all(|i| (lock[i] + key[i]) <= 7)
}

pub fn process(input: &str) -> usize {
    let (locks, keys) = parse(input);

    let result = locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| fits(lock, key))
        .count();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let result = process(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input);
        assert_eq!(result, 3338);
    }
}
