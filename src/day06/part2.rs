// use indicatif::ProgressIterator;

use super::walk::{SimulatedWalk, SimulationResult};

use super::map::Map;

pub fn process(input: &str) -> usize {
    let mut map = Map::parse(input);

    let mut initial_walk = SimulatedWalk::new(&map);
    initial_walk.simulate();

    let feasible_options = initial_walk
        .visited
        .keys()
        // .progress()
        .filter(|new_wall_location| {
            if new_wall_location != &&map.starting_location {
                map.set_wall(**new_wall_location, true);

                let mut walk = SimulatedWalk::new(&map);
                let walk_result = walk.simulate();

                map.set_wall(**new_wall_location, false);

                walk_result == SimulationResult::Loop
            } else {
                false
            }
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
