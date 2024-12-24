use std::{collections::HashMap, ops::Neg};

use glam::IVec2;
use itertools::Itertools;

pub fn process(input: &str) -> usize {
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
                    let d_len = d.x.abs() + d.y.abs();
                    let steps = (grid_size.x + grid_size.y) / d_len;
                    (steps.neg()..steps).map(move |i| l1 + i * d)
                })
                .filter(|loc| 
                    loc.x >= 0 && loc.y >= 0 && loc.x < grid_size.x && loc.y < grid_size.y
                )
        })
        .unique()
        .count();

    antinode_count
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

        let result = process(input);

        assert_eq!(34, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        assert_eq!(912, result);
    }
}
