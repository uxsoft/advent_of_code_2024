use super::shared::coordinate::Coordinate;
use super::shared::direction::Direction;
use std::collections::HashSet;

pub mod part1;
pub mod part2;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Node {
    pos: Coordinate,
    dir: Direction,
}

fn neighbors(spaces: &HashSet<Coordinate>, node: Node) -> Vec<(usize, Node)> {
    let mut n = Vec::with_capacity(2);

    n.push((
        1000,
        Node {
            pos: node.pos,
            dir: node.dir.rotate_counterclockwise(),
        },
    ));

    n.push((
        1000,
        Node {
            pos: node.pos,
            dir: node.dir.rotate_clockwise(),
        },
    ));

    if let Some(next_pos) = node.dir.advance(&node.pos) {
        if spaces.contains(&next_pos) {
            n.push((
                1,
                Node {
                    pos: next_pos,
                    dir: node.dir,
                },
            ));
        }
    }

    n
}

pub fn parse(input: &str) -> (Coordinate, Coordinate, HashSet<Coordinate>) {
    let mut start = Coordinate::new(0, 0);
    let mut end = Coordinate::new(0, 0);
    let mut spaces = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            match c {
                'S' => {
                    start = Coordinate::new(x, y);
                }
                'E' => {
                    end = Coordinate::new(x, y);
                }
                '.' => {
                    spaces.insert(Coordinate::new(x, y));
                }
                _ => (),
            }
        }
    }

    (start, end, spaces)
}
