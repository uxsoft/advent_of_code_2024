use std::collections::{HashSet, VecDeque};

use chumsky::container::Container;

#[derive(Debug)]
struct Region {
    id: char,
    area: usize,
    perimeter: usize,
}

fn neighbors(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x + 1 < grid[0].len() {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y + 1 < grid.len() {
        neighbors.push((x, y + 1));
    }

    neighbors
}

fn strongly_connected_component(
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
    start_x: usize,
    start_y: usize,
) -> Region {
    let mut candidates: VecDeque<(usize, usize)> = VecDeque::new();
    let id = grid[start_y][start_x];
    let mut area = 0;
    let mut perimeter = 0;

    candidates.push_back((start_x, start_y));

    while let Some((x, y)) = candidates.pop_front() {
        if !visited.contains(&(x, y)) {
            visited.push((x, y));
            let nb = neighbors(grid, x, y);

            area += 1;

            perimeter += nb.iter().filter(|(x, y)| grid[*y][*x] != id).count();

            if x == 0 {
                perimeter += 1;
            }
            if y == 0 {
                perimeter += 1;
            }
            if x == grid[0].len() - 1 {
                perimeter += 1;
            }
            if y == grid.len() - 1 {
                perimeter += 1;
            }

            nb.iter()
                .filter(|(x, y)| grid[*y][*x] == id)
                .for_each(|c| candidates.push_back(*c));
        }
    }

    Region {
        id,
        area,
        perimeter,
    }
}

pub fn process(input: &str) -> usize {
    let grid = super::parse(input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut regions = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !visited.contains(&(x, y)) {
                let region = strongly_connected_component(&grid, &mut visited, x, y);
                regions.push(region);
            }
        }
    }

    // dbg!(&regions);

    regions.iter().map(|r| r.area * r.perimeter).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trivial() {
        let input = "AAAA
BBCD
BBCC
EEEC";

        let result = process(input);

        assert_eq!(140, result);
    }

    #[test]
    fn example() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        let result = process(input);

        assert_eq!(1930, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        assert_eq!(1431316, result);
    }
}
