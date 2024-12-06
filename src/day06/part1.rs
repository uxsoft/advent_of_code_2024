use super::walk::SimulatedWalk;
use super::map::Map;

pub fn process(input: &str) -> usize {
    let map = Map::parse(input);
    let mut walk = SimulatedWalk::new(&map);

    walk.simulate();

    let visited_count = walk.visited.len();

    println!("{walk}");

    visited_count
}

#[cfg(test)]
mod tests {
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

        println!("{:?}", result);

        assert_eq!(41, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        println!("{:?}", result);

        assert_eq!(4826, result);
    }
}
