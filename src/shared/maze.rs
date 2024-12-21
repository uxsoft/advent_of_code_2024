use std::collections::{HashMap, HashSet, VecDeque};

use super::{coordinate::Coordinate, direction::Direction};

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub walls: HashSet<Coordinate>,
    pub spaces: HashSet<Coordinate>,
    pub start: Coordinate,
    pub end: Coordinate,
}

impl Maze {
    pub fn parse(input: &str) -> Self {
        let mut start = Coordinate::new(0, 0);
        let mut end = Coordinate::new(0, 0);
        let mut spaces = HashSet::new();
        let mut walls = HashSet::new();
        let height = input.lines().count();
        let width = input.lines().next().unwrap().len();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.char_indices() {
                match c {
                    'S' => {
                        start = Coordinate::new(x, y);
                        spaces.insert(Coordinate::new(x, y));
                    }
                    'E' => {
                        end = Coordinate::new(x, y);
                        spaces.insert(Coordinate::new(x, y));
                    }
                    '.' => {
                        spaces.insert(Coordinate::new(x, y));
                    }
                    '#' => {
                        walls.insert(Coordinate::new(x, y));
                    }
                    _ => (),
                }
            }
        }

        Maze {
            width,
            height,
            walls,
            spaces,
            start,
            end,
        }
    }

    pub fn neighbors(&self, from: &Coordinate) -> Vec<Coordinate> {
        Direction::all()
            .iter()
            .filter_map(|d| d.advance_bounded(from, self.width, self.height))
            .filter(|p| self.spaces.contains(p))
            .collect()
    }

    pub fn shortest_path(&self) -> (HashMap<Coordinate, usize>, HashMap<Coordinate, Coordinate>) {
        let mut dist: HashMap<Coordinate, usize> = HashMap::new();
        let mut prev: HashMap<Coordinate, Coordinate> = HashMap::new();
        let mut queue = VecDeque::new();

        dist.insert(self.start, 0);
        queue.push_back(self.start);

        while let Some(u) = queue.pop_front() {
            for v in self.neighbors(&u) {
                let alt = dist.get(&u).unwrap_or(&usize::MAX) + 1;
                if alt < *dist.get(&v).unwrap_or(&usize::MAX) {
                    dist.insert(v, alt);
                    prev.insert(v, u);
                    queue.push_back(v);
                }
            }
        }

        (dist, prev)
    }

    pub fn print(&self, highlights: &HashSet<Coordinate>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = Coordinate::new(x, y);
                let s = match (self.walls.contains(&c), highlights.contains(&c)) {
                    (true, _) => '#',
                    (false, true) => '.',
                    (false, false) => ' ',
                };

                print!("{}", s);
            }
            println!("");
        }
    }
}
