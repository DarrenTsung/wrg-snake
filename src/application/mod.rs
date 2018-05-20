use std::mem;
use wasm_rgame::Canvas;
use wasm_rgame::delegate_prelude::*;
use wrg_2d::{Direction, Grid, IntVector2};

mod snake_game;
use self::snake_game::{SnakeGame, SnakeGameState, SnakeGameHandle, Config, calculate_grid_canvas_size};

mod ui;
use self::ui::{new_button, TEXT_COLOR};
use wasm_rgame_ui::{Text, TextConfig, Vector2, ButtonHandle, Transform, TransformVector};

pub struct ApplicationDelegate {
    state: ApplicationState,
}

enum ApplicationState {
    Replaced,
    TitleScreen {
        play_button: SpawnHandle<ButtonHandle>,
        others: SpawnHandles,
    },

    Running {
        game: SpawnHandle<SnakeGameHandle>,
    },

    GameOver {
        play_again_button: SpawnHandle<ButtonHandle>,
        others: SpawnHandles,
    },
}

// TODO (darren): it would be nice to refactor this into multiple states
// right now, it seems like a big pain point is storing the SpawnHandle and also
// starting state between the spawned delegate and the handle (super annoying)
impl Delegate for ApplicationDelegate {
    fn tick(
        &mut self,
        context: &mut ApplicationContext,
        _key_manager: &KeyManager,
        _mouse_state: &MouseState,
        spawner: &mut DelegateSpawner,
    ) {
        self.state = match mem::replace(&mut self.state, ApplicationState::Replaced) {
            ApplicationState::Replaced => unreachable!(),
            ApplicationState::TitleScreen { play_button, others } => {
                if play_button.clicked() {
                    let game = spawner.spawn(SnakeGame::new(Self::config()));
                    ApplicationState::Running { game }
                } else {
                    ApplicationState::TitleScreen { play_button, others }
                }
            },
            ApplicationState::Running { mut game } => {
                let state = game.replace_state(SnakeGameState::Running);
                match state {
                    SnakeGameState::Finished { size, time } => {
                        // put game state back to normal every time as it will be passed
                        // to new state no matter what.
                        game.replace_state(SnakeGameState::Finished { size, time });

                        // spawn a new game if the game is over for X seconds
                        if context.total_s() - time > 1.0 {
                            Self::new_game_over(game, spawner, size)
                        } else {
                            ApplicationState::Running { game }
                        }
                    },
                    SnakeGameState::Running => {
                        // no need to replace back GameState as it's already Running
                        ApplicationState::Running { game }
                    }
                }
            },
            ApplicationState::GameOver { play_again_button, others } => {
                if play_again_button.clicked() {
                    let game = spawner.spawn(SnakeGame::new(Self::config()));
                    ApplicationState::Running { game }
                } else {
                    ApplicationState::GameOver { play_again_button, others }
                }
            }
        }
    }

    fn render(&self, graphics: &mut Graphics) {
        match self.state {
            ApplicationState::GameOver { .. } | ApplicationState::TitleScreen { .. } => {
                let canvas = Canvas::instance();

                // draw a transparent overlay over the game
                graphics.draw_rect(
                    0.0,
                    0.0,
                    canvas.width() as f32,
                    canvas.height() as f32,
                    [255, 255, 255, 150]
                );
            },
            _ => (),
        }
    }
}

impl ApplicationDelegate {
    pub fn new(spawner: &mut DelegateSpawner) -> ApplicationDelegate {
        let canvas = Canvas::instance();
        let (canvas_width, canvas_height) = calculate_grid_canvas_size(&Self::grid());
        canvas.set_width(canvas_width);
        canvas.set_height(canvas_height);

        ApplicationDelegate {
            state: Self::new_title_screen(spawner),
        }
    }

    fn config() -> Config {
        Config {
            start_position: IntVector2 { x: 0, y: 13 },
            start_length: 3,
            start_direction: Direction::Right,

            input_allowed: true,

            grid: Self::grid(),
        }
    }

    fn grid() -> Grid {
        Grid::new(15, 15)
    }

    fn new_title_screen(spawner: &mut DelegateSpawner) -> ApplicationState {
        let mut title_config = Self::config();
        title_config.input_allowed = false;
        let game_handle = spawner.spawn(SnakeGame::new(title_config));
        let play_button = spawner.spawn(new_button(Transform::new(
            TransformVector::Relative(Vector2 { x: 0.5, y: 0.29, }),
            TransformVector::Absolute(Vector2 { x: 150.0, y: 40.0, }),
            Vector2 { x: 0.5, y: 0.5, },
        ), "Play".to_string(), 23.0, 1));

        let title_handle = spawner.spawn(Text::new(Transform::new(
            TransformVector::Relative(Vector2 { x: 0.5, y: 0.66, }),
            TransformVector::Absolute(Vector2 { x: 10.0, y: 10.0, }),
            Vector2 { x: 0.5, y: 0.5, },
        ), TextConfig {
            text: "SNAKE-RS".to_string(),
            font_size: 60.0,
            color: TEXT_COLOR,
            render_order: 5,
        }));

        let subtitle_handle = spawner.spawn(Text::new(Transform::new(
            TransformVector::Relative(Vector2 { x: 0.5, y: 0.19 }),
            TransformVector::Absolute(Vector2 { x: 10.0, y: 10.0, }),
            Vector2 { x: 0.5, y: 0.5, },
        ), TextConfig {
            text: "WASD to control snake".to_string(),
            font_size: 13.0,
            color: TEXT_COLOR,
            render_order: 5,
        }));

        let made_with_handle = spawner.spawn(Text::new(Transform::new(
            TransformVector::Relative(Vector2 { x: 0.5, y: 0.54 }),
            TransformVector::Absolute(Vector2 { x: 10.0, y: 10.0, }),
            Vector2 { x: 0.5, y: 0.5, },
        ), TextConfig {
            text: "made with wasm-rgame".to_string(),
            font_size: 19.5,
            color: TEXT_COLOR,
            render_order: 5,
        }));

        // Join handles that will not be queried for information
        // for convenience
        let others = SpawnHandles::new()
            .with(game_handle)
            .with(title_handle)
            .with(subtitle_handle)
            .with(made_with_handle);

        ApplicationState::TitleScreen {
            play_button,
            others,
        }
    }

    fn new_game_over(
        game_handle: SpawnHandle<SnakeGameHandle>,
        spawner: &mut DelegateSpawner,
        size: usize
    ) -> ApplicationState {
        let play_again_button = spawner.spawn(new_button(Transform::new(
            TransformVector::Relative(Vector2 { x: 0.5, y: 0.30, }),
            TransformVector::Absolute(Vector2 { x: 150.0, y: 40.0, }),
            Vector2 { x: 0.5, y: 0.5, },
        ), "Play Again".to_string(), 23.0, 1));

        let header_handle = spawner.spawn(Text::new(Transform::new(
            TransformVector::Relative(Vector2 { x: 0.5, y: 0.63, }),
            TransformVector::Absolute(Vector2 { x: 10.0, y: 10.0, }),
            Vector2 { x: 0.5, y: 0.5, },
        ), TextConfig {
            text: "Game Over".to_string(),
            font_size: 60.0,
            color: TEXT_COLOR,
            render_order: 5,
        }));

        let game_info_handle = spawner.spawn(Text::new(Transform::new(
            TransformVector::Relative(Vector2 { x: 0.5, y: 0.20 }),
            TransformVector::Absolute(Vector2 { x: 10.0, y: 10.0, }),
            Vector2 { x: 0.5, y: 0.5, },
        ), TextConfig {
            text: format!("Your score is: {}", size - 3),
            font_size: 13.0,
            color: TEXT_COLOR,
            render_order: 5,
        }));

        let others = SpawnHandles::new()
            .with(game_handle)
            .with(header_handle)
            .with(game_info_handle);

        ApplicationState::GameOver {
            play_again_button,
            others,
        }
    }
}
