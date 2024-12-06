use glam::{ivec2, IVec2};

use super::direction::Direction;

#[derive(Debug)]
pub struct Map {
    pub walls: Vec<Vec<bool>>,
    pub starting_direction: Direction,
    pub starting_location: IVec2,
}

#[repr(u8)]
pub enum PeekResult {
    Free = 0,
    Wall = 1,
    OutOfBounds = 2,
}

impl Map {
    pub fn peek(&self, location: IVec2) -> PeekResult {
        let result = self
            .walls
            .get(location.y as usize)
            .and_then(|line| line.get(location.x as usize));

        match result {
            Some(false) => PeekResult::Free,
            Some(true) => PeekResult::Wall,
            None => PeekResult::OutOfBounds,
        }
    }

    pub fn set_wall(&mut self, location: IVec2, is_wall: bool) {
        self.walls[location.y as usize][location.x as usize] = is_wall;
    }

    pub fn parse(input: &str) -> Map {
        let mut player_location = ivec2(i32::MAX, i32::MAX);
    
        let walls: Vec<Vec<bool>> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => true,
                        '^' => {
                            player_location = ivec2(x as i32, y as i32);
                            false
                        }
                        _ => false,
                    })
                    .collect()
            })
            .collect();
    
    
        Map {
            walls,
            starting_direction: Direction::Up,
            starting_location: player_location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
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

        let result = Map::parse(input);

        println!("{:?}", result);
    }
}
