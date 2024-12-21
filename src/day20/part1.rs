use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::Itertools;

use super::super::shared::{coordinate::Coordinate, direction::Direction, maze::Maze};

pub fn process(input: &str, treshold: usize) -> usize {
    let maze = Maze::parse(input);

    let (dist, _) = maze.shortest_path();

    let cheats: Vec<_> = maze
        .spaces
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| a.distance(b) == 2.)
        .map(|(a, b)| dist[&a].abs_diff(dist[&b]) - 2) // takes two picoseconds to make the shortcut
        .filter(|d| *d >= treshold)
        .collect();

    cheats.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let result = process(input, 10);
        assert_eq!(result, 10);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input, 100);
        assert_eq!(result, 1337);
    }
}
