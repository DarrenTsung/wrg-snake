use circular_queue::CircularQueue;
use wrg_2d::IntVector2;

pub mod action;
pub use self::action::SnakeActionMoveTo;

pub mod controller;
pub use self::controller::PlayerSnakeController;

// we should never hit this, but it's nice to have some bounds
const MAX_LENGTH : usize = 1000;

pub struct Snake {
    position_history: CircularQueue<IntVector2>,
    start_position: IntVector2,
    length: usize,
}

impl Snake {
    pub fn new(start_position: IntVector2, start_length: usize) -> Snake {
        let mut position_history = CircularQueue::with_capacity(MAX_LENGTH);
        position_history.push(start_position);

        let mut snake = Snake {
            start_position,
            position_history,
            length: 1,
        };

        for _ in 1..start_length {
            snake.grow();
        }

        snake
    }

    pub fn grow(&mut self) {
        debug_assert!(self.length < MAX_LENGTH);
        self.length += 1;
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn move_to(&mut self, position: IntVector2) {
        // right now it's not supported to move to the same position,
        // although I could see a possible use-case for compacting the snake into
        // a single position after some effect (teleport?)
        debug_assert!(self.current_position() != position);

        self.position_history.push(position);
    }

    pub fn current_position(&self) -> IntVector2 {
        self.position_history.iter().next().expect("Always at least one position").clone()
    }

    pub fn positions(&self) -> impl Iterator<Item=&IntVector2> {
        let repeat = self.length.saturating_sub(self.position_history.len());
        self.position_history.iter().take(self.length())
            // if length() is larger than position history, then we repeat the start position
            .chain(repeat_ref(&self.start_position).take(repeat))
    }
}

fn repeat_ref<'a, T: 'a>(item: &T) -> RepeatRef<T> {
    RepeatRef { item }
}

struct RepeatRef<'a, T: 'a> {
    item: &'a T,
}

impl<'a, T: 'a> Iterator for RepeatRef<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_long_and_move_around() {
        let mut snake = Snake::new(IntVector2 { x: 0, y: 0 }, 3);
        snake.move_to(IntVector2 { x: 1, y: 0 });
        snake.move_to(IntVector2 { x: 2, y: 0 });
        snake.move_to(IntVector2 { x: 3, y: 0 });
        snake.move_to(IntVector2 { x: 4, y: 0 });
        snake.move_to(IntVector2 { x: 5, y: 0 });
        snake.move_to(IntVector2 { x: 6, y: 0 });

        for position in snake.positions() {
            println!("position: {:?}", position);
        }
    }

    #[test]
    fn grow_can_go_beyond_history() {
        let mut snake = Snake::new(IntVector2 { x: 0, y: 0 }, 1);
        snake.move_to(IntVector2 { x: 1, y: 0 });
        snake.move_to(IntVector2 { x: 2, y: 0 });

        for _ in 0..5 { snake.grow(); }

        assert_eq!(snake.length(), 6);
        let mut positions = snake.positions();
        assert_eq!(positions.next(), Some(&IntVector2 { x: 2, y: 0 }));
        assert_eq!(positions.next(), Some(&IntVector2 { x: 1, y: 0 }));
        assert_eq!(positions.next(), Some(&IntVector2 { x: 0, y: 0 }));
        assert_eq!(positions.next(), Some(&IntVector2 { x: 0, y: 0 }));
        assert_eq!(positions.next(), Some(&IntVector2 { x: 0, y: 0 }));
        assert_eq!(positions.next(), Some(&IntVector2 { x: 0, y: 0 }));
        assert_eq!(positions.next(), None);
    }

    #[test]
    fn normal_condition_test_everything() {
        let mut snake = Snake::new(IntVector2 { x: 0, y: 0 }, 1);
        assert_eq!(snake.length(), 1);
        assert_eq!(snake.current_position(), IntVector2 { x: 0, y: 0 });

        snake.move_to(IntVector2 { x: 1, y: 0 });
        assert_eq!(snake.length(), 1);
        assert_eq!(snake.current_position(), IntVector2 { x: 1, y: 0 });

        snake.grow();
        assert_eq!(snake.length(), 2);
        assert_eq!(snake.current_position(), IntVector2 { x: 1, y: 0 });

        snake.grow();
        assert_eq!(snake.length(), 3);
        assert_eq!(snake.current_position(), IntVector2 { x: 1, y: 0 });

        snake.move_to(IntVector2 { x: 201, y: -30 });
        assert_eq!(snake.length(), 3);
        assert_eq!(snake.current_position(), IntVector2 { x: 201, y: -30 });
    }
}
