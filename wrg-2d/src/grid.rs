use vector::IntVector2;

#[derive(Clone, Copy)]
pub struct Grid {
    pub width: u16,
    pub height: u16,
}

impl Grid {
    pub fn new(width: u16, height: u16) -> Grid {
        Grid { width, height, }
    }

    pub fn contains(&self, position: IntVector2) -> bool {
        position.x >= 0 && position.x < (self.width as i32) &&
        position.y >= 0 && position.y < (self.height as i32)
    }

    pub fn wrap(&self, position: IntVector2) -> IntVector2 {
        IntVector2 {
            x: position.x.mod_euc(self.width as i32),
            y: position.y.mod_euc(self.height as i32),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_works_as_expected() {
        let grid = Grid::new(5, 5);
        assert!(grid.contains(IntVector2 { x: 0, y: 0 }));
        assert!(grid.contains(IntVector2 { x: 4, y: 4 }));
        assert!(grid.contains(IntVector2 { x: 0, y: 4 }));
        assert!(grid.contains(IntVector2 { x: 4, y: 0 }));
        assert!(grid.contains(IntVector2 { x: 2, y: 2 }));
        assert!(grid.contains(IntVector2 { x: 4, y: 1 }));

        // doesn't contains
        assert!(!grid.contains(IntVector2 { x: -1, y: 0 }));
        assert!(!grid.contains(IntVector2 { x: 0, y: 5 }));
        assert!(!grid.contains(IntVector2 { x: 2, y: 10 }));
        assert!(!grid.contains(IntVector2 { x: 30, y: 1 }));
    }

    #[test]
    fn wrap_works_as_expected() {
        let grid = Grid::new(5, 5);
        assert_eq!(grid.wrap(IntVector2 { x: 0, y: 0 }), IntVector2 { x: 0, y: 0 });
        assert_eq!(grid.wrap(IntVector2 { x: 4, y: 4 }), IntVector2 { x: 4, y: 4 });
        assert_eq!(grid.wrap(IntVector2 { x: 0, y: 4 }), IntVector2 { x: 0, y: 4 });
        assert_eq!(grid.wrap(IntVector2 { x: 4, y: 0 }), IntVector2 { x: 4, y: 0 });
        assert_eq!(grid.wrap(IntVector2 { x: -2, y: 0 }), IntVector2 { x: 3, y: 0 });
        assert_eq!(grid.wrap(IntVector2 { x: 1, y: 5 }), IntVector2 { x: 1, y: 0 });
        assert_eq!(grid.wrap(IntVector2 { x: -1, y: 8 }), IntVector2 { x: 4, y: 3 });
    }
}
