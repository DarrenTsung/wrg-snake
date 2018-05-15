use vector::IntVector2;

/// Cardinal Direction enum for 2D
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn to_position(&self) -> IntVector2 {
        match self {
            Direction::Up => IntVector2 { x: 0, y: 1 },
            Direction::Right => IntVector2 { x: 1, y: 0 },
            Direction::Down => IntVector2 { x: 0, y: -1 },
            Direction::Left => IntVector2 { x: -1, y: 0 },
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
        }
    }
}
