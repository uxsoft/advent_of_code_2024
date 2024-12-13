use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Region {
    id: char,
    members: HashSet<(usize, usize)>,
}

impl Region {
    fn area(&self) -> usize {
        self.members.len()
    }

    fn sides(&self, grid: &Vec<Vec<char>>) -> usize {
        let mut corners = 0;
        for (x, y) in &self.members {
            let nb_count = neighbors(grid, *x, *y)
                .iter()
                .filter(|(nx, ny)| self.members.contains(&(*nx, *ny)))
                .count();

            let n = *y > 0 && self.members.contains(&(*x, y - 1));
            let s = *y + 1 < grid.len() && self.members.contains(&(*x, y + 1));
            let w = *x > 0 && self.members.contains(&(x - 1, *y));
            let e = *x + 1 < grid[0].len() && self.members.contains(&(x + 1, *y));

            let nw = *y > 0 && *x > 0 && self.members.contains(&(x - 1, y - 1));
            let ne = *y > 0 && *x + 1 < grid[0].len() && self.members.contains(&(x + 1, y - 1));
            let sw = *y + 1 < grid.len() && *x > 0 && self.members.contains(&(x - 1, y + 1));
            let se = *y + 1 < grid.len()
                && *x + 1 < grid[0].len()
                && self.members.contains(&(x + 1, y + 1));

            // Outside corners
            corners += match nb_count {
                0 => 4,
                1 => 2,
                2 if n && s || e && w => 0,
                2 => 1,
                _ => 0,
            };

            // Add inner corners
            for (left, mid, right) in [(n, ne, e), (e, se, s), (s, sw, w), (w, nw, n)] {
                corners += match (left, mid, right) {
                    (true, false, true) => 1,
                    _ => 0,
                }
            }
        }

        corners
    }
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
    let mut members = HashSet::new();
    let id = grid[start_y][start_x];

    candidates.push_back((start_x, start_y));

    while let Some((x, y)) = candidates.pop_front() {
        if !visited.contains(&(x, y)) {
            visited.insert((x, y));
            members.insert((x, y));

            let nb = neighbors(grid, x, y);

            nb.iter()
                .filter(|(x, y)| grid[*y][*x] == id)
                .for_each(|c| candidates.push_back(*c));
        }
    }

    Region { id, members }
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

    regions.iter().map(|r| r.area() * r.sides(&grid)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trivial1() {
        let input = "AAAA
BBCD
BBCC
EEEC";

        let result = process(input);

        assert_eq!(80, result);
    }

    #[test]
    fn trivial2() {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

        let result = process(input);

        assert_eq!(436, result);
    }

    #[test]
    fn trivial3() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

        let result = process(input);

        assert_eq!(236, result);
    }

    #[test]
    fn trivial4() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

        let result = process(input);

        assert_eq!(368, result);
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

        assert_eq!(1206, result);
    }

    #[test]
    fn real() {
        let input = include_str!("input.txt");

        let result = process(input);

        assert_eq!(821428, result);
    }
}
