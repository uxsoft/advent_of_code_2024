use glam::{ivec2, IVec2};
use std::collections::HashMap;

pub mod part1;
pub mod part2;

pub fn parse(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| match ch {
                    '.' => None,
                    c => Some((ivec2(x as i32, y as i32), c)),
                })
        })
        .collect()
}

pub fn grid_size(input: &str) -> IVec2 {
    let width = input.lines().nth(0).unwrap().len();
    let height = input.lines().count();
    ivec2(width as i32, height as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let result = parse(input);

        println!("{:?}", result);

        assert_eq!(7, result.len());
    }
}
