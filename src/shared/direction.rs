use super::coordinate::Coordinate;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn next(&self, current: &Coordinate) -> Option<Coordinate> {
        match self {
            Direction::North if current.y > 0 => Some(Coordinate::new(current.x, current.y - 1)),
            Direction::South => Some(Coordinate::new(current.x, current.y + 1)),
            Direction::East => Some(Coordinate::new(current.x + 1, current.y)),
            Direction::West if current.x > 0 => Some(Coordinate::new(current.x - 1, current.y)),
            _ => None,
        }
    }

    pub fn next_in_bounds(
        &self,
        current: &Coordinate,
        width: usize,
        height: usize,
    ) -> Option<Coordinate> {
        match self {
            Direction::North if current.y > 0 => Some(Coordinate::new(current.x, current.y - 1)),
            Direction::South if current.y < height - 1 => {
                Some(Coordinate::new(current.x, current.y + 1))
            }
            Direction::West if current.x > 0 => Some(Coordinate::new(current.x - 1, current.y)),
            Direction::East if current.x < width - 1 => {
                Some(Coordinate::new(current.x + 1, current.y))
            }
            _ => None,
        }
    }
}
