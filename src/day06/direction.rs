use glam::{ivec2, IVec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn forward(&self) -> IVec2 {
        use Direction::*;

        match self {
            Up => ivec2(0, -1),
            Right => ivec2(1, 0),
            Down => ivec2(0, 1),
            Left => ivec2(-1, 0),
        }
    }

    pub fn rotate_90_right(&self) -> Direction {
        use Direction::*;

        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}