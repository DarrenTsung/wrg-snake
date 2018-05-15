use wrg_2d::{IntVector2, Direction, Grid};

pub struct Config {
    pub start_position: IntVector2,
    pub start_length: usize,
    pub start_direction: Direction,

    pub input_allowed: bool,

    pub grid: Grid,
}
