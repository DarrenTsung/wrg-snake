use super::Snake;
use wasm_rgame::{KeyManager, key_codes};
use wrg_2d::{IntVector2, Direction};

pub struct SnakeActionMoveTo {
    pub position: IntVector2
}

pub struct PlayerSnakeController {
    direction: Direction,
    next_direction: Option<Direction>,
    buffered_direction: Option<Direction>,
}

impl PlayerSnakeController {
    pub fn new(start_direction: Direction) -> PlayerSnakeController {
        PlayerSnakeController {
            direction: start_direction,
            next_direction: None,
            buffered_direction: None,
        }
    }

    pub fn store_direction_change(&mut self, key_manager: &KeyManager) {
        // Don't let direction change if already going opposite direction
        let wanted_direction = if key_manager.key_down(key_codes::W) {
            Direction::Up
        } else if key_manager.key_down(key_codes::D) {
            Direction::Right
        } else if key_manager.key_down(key_codes::S) {
            Direction::Down
        } else if key_manager.key_down(key_codes::A) {
            Direction::Left
        } else {
            return;
        };

        let valid = self.is_valid(wanted_direction);
        if valid {
            self.next_direction = Some(wanted_direction);
        }

        // In an effort to make tight turns easier, we store an additional "buffered"
        // direction change if we have a `next_direction` and it's invalid for the current direction
        if !valid && self.next_direction.is_some() {
            self.buffered_direction = Some(wanted_direction);
        }
    }

    pub fn change_direction(&mut self) {
        if let Some(direction) = self.next_direction.take() {
            self.direction = direction;
        }

        if let Some(buffered_direction) = self.buffered_direction.take() {
            if self.is_valid(buffered_direction) {
                self.next_direction = Some(buffered_direction);
            }
        }
    }

    fn is_valid(&self, new_direction: Direction) -> bool {
        self.direction.opposite() != new_direction
    }

    pub fn action(&self, snake: &Snake) -> SnakeActionMoveTo {
        let new_position = snake.current_position() + self.direction.to_position();
        SnakeActionMoveTo { position: new_position }
    }
}
