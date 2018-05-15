use super::TEXT_COLOR;
use wasm_rgame_ui::{ButtonConfig, Button, TextConfig, Transform};

const HOVERED_COLOR: [u8; 4] = [187, 173, 158, 255];
const CLICKED_COLOR: [u8; 4] = [175, 161, 146, 255];
const COLOR: [u8; 4] = [190, 176, 161, 255];

pub fn new_button(transform: Transform, text: String, font_size: f32, render_order: i32) -> Button {
    let mut button = Button::new(transform, ButtonConfig {
        hovered_color: HOVERED_COLOR,
        clicked_color: CLICKED_COLOR,
        color: COLOR,

        render_order,
    });

    button.set_text(TextConfig {
        text,
        font_size,
        color: TEXT_COLOR,

        render_order: render_order + 1,
    });

    button
}
