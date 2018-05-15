use std::rc::Rc;
use super::{Snake, Grid, Food, PsuedoRandom};
use wasm_rgame::{Graphics};

const CELL_SIZE : u16 = 20;
const CELL_PADDING : u16 = 2;

const GRID_PADDING : u16 = 10;

lazy_static! {
    // Too lazy to remove this from lazy_static in case
    // we want to modify the colors dynamically
    static ref GRID_MARKER_COLOR : Vec<[u8; 4]> = {
        let color = vec![
            [233, 182, 169, 80],
            [253, 196, 172, 80],
            [235, 207, 178, 80],
        ];
        color
    };
}

const FOOD_COLOR: [u8; 4] = [179, 141, 151, 255];
const SNAKE_COLOR: [u8; 4] = [66, 75, 84, 255];

pub fn calculate_grid_canvas_size(grid: &Grid) -> (u32, u32) {
    // Assign the canvas sizing
    let grid_width = (grid.width * (CELL_SIZE + CELL_PADDING)) - CELL_PADDING + (2 * GRID_PADDING);
    let grid_height = (grid.height * (CELL_SIZE + CELL_PADDING)) - CELL_PADDING + (2 * GRID_PADDING);
    (grid_width.into(), grid_height.into())
}

pub struct SnakeGameRenderer {
    random: Rc<PsuedoRandom>,
}

impl SnakeGameRenderer {
    pub fn new(random: Rc<PsuedoRandom>) -> SnakeGameRenderer {
        SnakeGameRenderer {
            random,
        }
    }

    pub fn render(
        &self,
        grid: &Grid,
        snake: &Snake,
        foods: &Vec<Food>,
        graphics: &mut Graphics,
    )
    {
        // Draw the grid markers
        let mut index = 0;
        for x in 0..grid.width {
            for y in 0..grid.height {
                let color_index = (self.random.hash_value(index) as usize) % GRID_MARKER_COLOR.len();
                let color = GRID_MARKER_COLOR[color_index];
                Self::draw_cell(graphics, x, y, color);
                index += 1;
            }
        }

        // Draw the foods
        for food in foods {
            let pos = food.pos();
            Self::draw_cell(graphics, pos.x as u16, pos.y as u16, FOOD_COLOR);
        }

        // Draw the snake
        for position in snake.positions() {
            Self::draw_cell(graphics, position.x as u16, position.y as u16, SNAKE_COLOR);
        }
    }

    fn draw_cell(graphics: &mut Graphics, x: u16, y: u16, color: [u8; 4]) {
        let pos_x = GRID_PADDING + (x * CELL_SIZE) + (x * CELL_PADDING);
        let pos_y = GRID_PADDING + (y * CELL_SIZE) + (y * CELL_PADDING);
        let width = CELL_SIZE;
        let height = CELL_SIZE;

        graphics.draw_rect(pos_x as f32, pos_y as f32, width as f32, height as f32, color);
    }
}
