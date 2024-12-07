// use indicatif::ProgressIterator;

use super::map::Map;
use super::walk::{SimulatedWalk, SimulationResult};
use rayon::prelude::*;

pub fn process(input: &str) -> usize {
    let map = Map::parse(input);

    let mut initial_walk = SimulatedWalk::new(&map);
    initial_walk.simulate();

    let feasible_options = initial_walk
        .visited
        .into_par_iter()
        .map(|(k, _)| k)
        .filter(|extra_wall| extra_wall != &map.starting_location)
        .filter(|extra_wall| {
            let mut walk = SimulatedWalk::new(&map);
            let walk_result = walk.simulate_with(*extra_wall);

            walk_result == SimulationResult::Loop
        })
        .count();

    feasible_options
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let result = process(input);
        assert_eq!(6, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);
        assert_eq!(1721, result);
    }
}
