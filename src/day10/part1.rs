use std::collections::{HashSet, VecDeque};

use chumsky::container::Container;
use itertools::Itertools;

fn successors(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut options = Vec::new();

    if x + 1 < grid[0].len() {
        options.push((x + 1, y));
    };

    if x > 0 {
        options.push((x - 1, y));
    };

    if y + 1 < grid.len() {
        options.push((x, y + 1));
    };

    if y > 0 {
        options.push((x, y - 1));
    };

    options
}

fn walk_trail(grid: &Vec<Vec<u8>>, start_x: usize, start_y: usize) -> usize {
    let mut visited = HashSet::new();
    let mut candidates = VecDeque::new();
    let mut peaks = 0;

    candidates.push_back((start_x, start_y, 0));

    while let Some((x, y, height)) = candidates.pop_front() {
        if height == grid[y][x] && !visited.contains(&(x, y)) {
            visited.push((x, y));

            if height == 9 {
                peaks += 1;
            } else {
                for (cx, cy) in successors(grid, x, y) {
                    candidates.push_back((cx, cy, height + 1));
                }
            }
        }
    }

    peaks
}

pub fn process(input: &str) -> usize {
    let map = super::parse(input);

    let score = (0..map.len())
        .cartesian_product(0..map[0].len())
        .filter(|(x, y)| map[*y][*x] == 0)
        .map(|(x, y)| walk_trail(&map, x, y))
        .sum();

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial1() {
        let input = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

        let result = process(input);

        assert_eq!(2, result);
    }

    #[test]
    fn trivial2() {
        let input = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

        let result = process(input);

        assert_eq!(4, result);
    }

    #[test]
    fn trivial3() {
        let input = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";

        let result = process(input);

        assert_eq!(3, result);
    }

    #[test]
    fn example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let result = process(input);

        assert_eq!(36, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        assert_eq!(472, result);
    }
}
