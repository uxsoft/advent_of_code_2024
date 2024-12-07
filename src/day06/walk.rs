use std::{
    collections::HashMap,
    fmt::Display,
};

use glam::{ivec2, IVec2};
use itertools::Itertools;

use super::{
    direction::Direction,
    map::{Map, PeekResult},
};

#[derive(PartialEq, Debug)]
#[repr(u8)]
pub enum SimulationResult {
    OutOfBounds = 0,
    Loop = 1,
}

pub struct SimulatedWalk<'a> {
    pub map: &'a Map,
    pub visited: HashMap<IVec2, Vec<Direction>>,
    pub player_location: IVec2,
    pub player_direction: Direction,
}

impl<'a> SimulatedWalk<'a> {
    pub fn new(map: &'a Map) -> Self {
        Self {
            map,
            visited: HashMap::new(),
            player_direction: map.starting_direction,
            player_location: map.starting_location,
        }
    }

    pub fn mark_visited(&mut self) {
        self.visited
            .entry(self.player_location)
            .and_modify(|v| v.push(self.player_direction))
            .or_insert(vec![self.player_direction]);
    }

    pub fn has_visited(&self, loc: &IVec2, dir: &Direction) -> bool {
        self.visited
            .get(loc)
            .map(|v| v.contains(dir))
            .unwrap_or(false)
    }

    pub fn simulate(&mut self) -> SimulationResult {
        self.simulate_with(ivec2(-1, -1))
    }

    pub fn simulate_with(&mut self, extra_wall: IVec2) -> SimulationResult {
        loop {
            self.mark_visited();

            let next_location = self.player_location + self.player_direction.forward();

            if self.has_visited(&next_location, &self.player_direction) {
                return SimulationResult::Loop;
            }

            match self.map.peek_with(next_location, extra_wall) {
                PeekResult::Free => {
                    self.player_location = next_location;
                }
                PeekResult::Wall => {
                    self.player_direction = self.player_direction.rotate_90_right();
                }
                PeekResult::OutOfBounds => {
                    return SimulationResult::OutOfBounds;
                }
            }
        }
    }
}

impl<'a> Display for SimulatedWalk<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .map
            .walls
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, wall)| {
                        match (*wall, self.visited.contains_key(&ivec2(x as i32, y as i32))) {
                            (true, _) => '#',
                            (false, true) => 'X',
                            (false, false) => '.',
                        }
                    })
                    .join("")
            })
            .join("\n");

        write!(f, "{str}")
    }
}
