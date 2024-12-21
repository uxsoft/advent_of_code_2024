use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::Itertools;

use super::super::shared::{coordinate::Coordinate, direction::Direction, maze::Maze};

pub fn process(input: &str, cheat_distance: usize, treshold: usize) -> usize {
    let maze = Maze::parse(input);

    let (dist, _) = maze.shortest_path();

    let cheats: Vec<_> = maze
        .spaces
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| a.axial_distance(b) <= cheat_distance)
        .map(|(a, b)| dist[&a].abs_diff(dist[&b]) - a.axial_distance(b)) // takes {distance} picoseconds to make the shortcut
        .filter(|d| *d >= treshold)
        .collect();

    // let g_cheats: HashSet<_> = cheats.iter().copied().collect();
    // dbg!(g_cheats);

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
        let result = process(input, 20, 50);
        assert_eq!(result, 285);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");
        let result = process(input, 20, 100);
        assert_eq!(result, 987695);
    }
}
