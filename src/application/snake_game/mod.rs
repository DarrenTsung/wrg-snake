use std::cell::RefCell;
use std::rc::Rc;
use wasm_rgame::delegate_prelude::*;
use wbg_rand::{Rng, wasm_rng};
use wrg_2d::{Grid, IntVector2};

mod config;
mod food;
mod renderer;
mod snake;

pub use self::config::Config;

use self::food::Food;
use self::renderer::SnakeGameRenderer;
use self::snake::{PlayerSnakeController, Snake, SnakeActionMoveTo};

pub use self::renderer::calculate_grid_canvas_size;

// Because this is a quick demo project, the goal is to get to
// interesting gameplay fast. Therefore the ramp-up time is
// fairly short :).
const ACTION_INTERVAL_BASE : f64 = 0.13;
const ACTION_INTERVAL_MIN : f64 = 0.05;

const ACTION_INTERVAL_DECR_PERCENT : f64 = 0.8;

pub struct SnakeGame {
    handle: SnakeGameHandle,
    grid: Grid,
    snake: Snake,
    foods: Vec<Food>,
    snake_controller: PlayerSnakeController,
    renderer: SnakeGameRenderer,
    input_allowed: bool,

    /// Last interval this SnakeGame saw
    last_interval: f64,
    interval_time: f64,
}

#[derive(Clone)]
pub struct SnakeGameHandle {
    state: Rc<RefCell<SnakeGameState>>,
}

pub enum SnakeGameState {
    Running,
    Finished {
        size: usize,
        time: f64,
    },
}

impl SnakeGame {
    pub fn new(config: Config) -> SnakeGame {
        let Config {
            start_position, start_length, start_direction, input_allowed, grid,
        } = config;

        SnakeGame {
            handle: SnakeGameHandle { state: Rc::new(RefCell::new(SnakeGameState::Running)) },
            grid,
            snake: Snake::new(start_position, start_length),
            foods: Vec::new(),
            snake_controller: PlayerSnakeController::new(start_direction),
            renderer: SnakeGameRenderer::new(),
            input_allowed,
            last_interval: 0.0,
            interval_time: ACTION_INTERVAL_BASE,
        }
    }

    fn generate_foods(&mut self) {
        if !self.foods.is_empty() {
            return;
        }

        loop {
            let pos = self.random_grid_position();

            let position_already_taken = {
                let mut collidable_positions = self.snake.positions()
                    .chain(self.foods.iter().map(|f| f.pos()));
                collidable_positions.any(|p| *p == pos)
            };

            if position_already_taken {
                continue;
            }

            self.foods.push(Food::new(pos));
            break;
        }
    }

    fn random_grid_position(&self) -> IntVector2 {
        let x = wasm_rng().gen_range(0, self.grid.width as usize) as i32;
        let y = wasm_rng().gen_range(0, self.grid.height as usize) as i32;
        IntVector2 { x, y }
    }
}

impl Delegate for SnakeGame {
    fn tick(
        &mut self,
        context: &mut ApplicationContext,
        key_manager: &KeyManager,
        _mouse_state: &MouseState,
        _delegate_spawner: &mut DelegateSpawner,
    ) {
        let new_state = match self.handle.state.replace(SnakeGameState::Running) {
            SnakeGameState::Running => {
                self.generate_foods();

                self.snake_controller.store_direction_change(key_manager);

                let mut state;
                let diff = context.total_s() - self.last_interval;

                // Instead of making up for lost intervals, let's just do actions if
                // at least one interval has passed. We don't want to jump intervals
                // even if the game is laggy. This is exploitable, but that's okay.
                if diff > self.interval_time {
                    self.last_interval = context.total_s();

                    if self.input_allowed {
                        // Only change the direction once per interval
                        self.snake_controller.change_direction();
                    }

                    let SnakeActionMoveTo { position } = self.snake_controller.action(&self.snake);
                    // If the snake goes off the grid, we wrap it around
                    let position = self.grid.wrap(position);

                    let prev_food_len = self.foods.len();
                    // remove all foods that collide with the new position
                    self.foods.retain(|food| *food.pos() != position);
                    let eaten_count = prev_food_len.saturating_sub(self.foods.len());
                    for _ in 0..eaten_count {
                        // decrease interval time each time snake grows
                        self.interval_time = (self.interval_time * ACTION_INTERVAL_DECR_PERCENT).max(ACTION_INTERVAL_MIN);
                        self.snake.grow();
                    }

                    let collided_with_self = self.snake.positions().any(|p| *p == position);
                    if collided_with_self {
                        state = SnakeGameState::Finished {
                            size: self.snake.length(),
                            time: context.total_s(),
                        };
                    } else {
                        self.snake.move_to(position);
                        state = SnakeGameState::Running;
                    }
                } else {
                    state = SnakeGameState::Running
                }

                state
            },
            finished => finished,
        };
        *(self.handle.state.borrow_mut()) = new_state;
    }

    fn render(&self, graphics: &mut Graphics) {
        self.renderer.render(&self.grid, &self.snake, &self.foods, graphics);
    }

    // render behind things
    fn render_order(&self) -> i32 { -1 }
}

impl SpawnableDelegate for SnakeGame {
    type Handle = SnakeGameHandle;

    fn handle(&self) -> Self::Handle {
        self.handle.clone()
    }
}

impl SnakeGameHandle {
    pub fn replace_state(&mut self, state: SnakeGameState) -> SnakeGameState {
        self.state.replace(state)
    }
}
