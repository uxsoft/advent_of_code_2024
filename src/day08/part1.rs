use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;

pub fn process(input: &str) -> miette::Result<usize> {
    let grid_size = super::grid_size(input);
    let coords_to_antenna = super::parse(input);
    let antenna_to_coords: HashMap<char, Vec<IVec2>> =
        coords_to_antenna
            .iter()
            .fold(HashMap::new(), |mut map, (loc, signal)| {
                map.entry(*signal)
                    .and_modify(|v| v.push(*loc))
                    .or_insert(vec![*loc]);
                map
            });

    let antinode_count = antenna_to_coords
        .values()
        .flat_map(|antenna| {
            antenna
                .into_iter()
                .tuple_combinations()
                .flat_map(|(l1, l2)| {
                    let d = l2 - l1;
                    vec![l2 + d, l1 - d]
                })
                .filter(|loc| loc.x >= 0 && loc.y >= 0 && loc.x < grid_size.x && loc.y < grid_size.y)
        })
        .unique()
        .count();

    Ok(antinode_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
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

        let result = process(input).unwrap();

        assert_eq!(14, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input).unwrap();

        assert_eq!(244, result);
    }
}
