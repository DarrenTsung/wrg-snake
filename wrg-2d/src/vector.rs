use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct IntVector2 {
    pub x: i32,
    pub y: i32,
}

impl Add for IntVector2 {
    type Output = IntVector2;

    fn add(self, other: IntVector2) -> IntVector2 {
        IntVector2 {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for IntVector2 {
    type Output = IntVector2;

    fn sub(self, other: IntVector2) -> IntVector2 {
        IntVector2 {x: self.x - other.x, y: self.y - other.y}
    }
}
