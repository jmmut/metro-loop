pub mod logic {
    pub mod constraints;
    pub mod grid;
    pub mod intersection;
    pub mod rails;
}
pub mod render;
pub mod sound;

pub mod scenes {
    pub mod main_menu;
    pub use main_menu::main_menu;
    pub mod loading_screen;
    pub use loading_screen::loading_screen;
    pub mod play;
    pub use play::play;
}

use crate::logic::constraints::{Constraints, RailCoord};
use crate::logic::grid::{get, Grid};
use juquad::widgets::anchor::Vertical;
use juquad::widgets::{StateStyle, Style};
use macroquad::prelude::*;

pub const DEFAULT_SHOW_SOLUTION: bool = false;
pub const SEE_SOLUTION_DURING_GAME: bool = true;
pub const VISUALIZE: bool = false;
pub const STEP_GENERATION: bool = false;
pub const SHOW_FPS: bool = false;

pub const BACKGROUND: Color = Color::new(0.1, 0.1, 0.1, 1.00);
pub const BACKGROUND_2: Color = Color::new(0.05, 0.05, 0.05, 1.00);
pub const PANEL_BACKGROUND: Color = LIGHTGRAY;

pub const FAILING: Color = ORANGE;
pub const SUCCESS: Color = Color::new(0.10, 0.75, 0.19, 1.00); // less saturated GREEN
pub const FAILING_DARK: Color = color_average(FAILING, BLACK);
pub const SUCCESS_DARK: Color = color_average(SUCCESS, BLACK);

pub const TRIANGLE: Color = Color::new(0.40, 0.7, 0.9, 1.00); // darker sky blue
pub const TRIANGLE_BORDER: Color = color_average_weight(BLACK, BLUE, 0.25);
pub const RAIL: Color = TRIANGLE;
pub const UNREACHABLE_RAIL: Color = FAILING;

pub const ENABLED_CELL: Color = BLUE;
pub const DISABLED_CELL: Color = DARKGRAY;
pub const HOVERED_CELL: Color = color_average(ENABLED_CELL, DISABLED_CELL);

pub const STYLE: Style = Style {
    at_rest: StateStyle {
        bg_color: color_average(LIGHTGRAY, WHITE),
        text_color: BLACK,
        border_color: DARKGRAY,
    },
    hovered: StateStyle {
        bg_color: WHITE,
        text_color: BLACK,
        border_color: LIGHTGRAY,
    },
    pressed: StateStyle {
        bg_color: GRAY,
        text_color: WHITE,
        border_color: DARKGRAY,
    },
};

pub const FONT_SIZE: f32 = 16.0;

pub const NUM_ROWS: i32 = 10;
pub const NUM_COLUMNS: i32 = 11;
pub const MAX_CELLS: usize = ((NUM_ROWS - 2) * (NUM_COLUMNS - 2)) as usize / 2;
pub const CLUE_PERCENTAGE: u32 = 30;

pub const CELL_WIDTH: f32 = 50.0;
pub const CELL_HEIGHT: f32 = 50.0;
pub const CELL_PAD: f32 = 5.0;
pub const GRID_PAD: f32 = 30.0;
pub const BUTTON_PANEL_WIDTH: f32 = 300.0;

pub const DEFAULT_WINDOW_WIDTH: i32 = (grid_width() + BUTTON_PANEL_WIDTH + 3.0 * GRID_PAD) as i32;
pub const DEFAULT_WINDOW_HEIGHT: i32 = (grid_height() + 2.0 * GRID_PAD) as i32;
pub const DEFAULT_WINDOW_TITLE: &str = "Metro Loop";

pub type AnyError = Box<dyn std::error::Error>;

pub const fn grid_width() -> f32 {
    (CELL_WIDTH + CELL_PAD) * NUM_COLUMNS as f32 - CELL_PAD
}
pub const fn grid_height() -> f32 {
    (CELL_HEIGHT + CELL_PAD) * NUM_ROWS as f32 - CELL_PAD
}

const fn color_average(color_1: Color, color_2: Color) -> Color {
    color_average_weight(color_1, color_2, 0.5)
}
const fn color_average_weight(color_1: Color, color_2: Color, weight: f32) -> Color {
    Color::new(
        color_1.r * (1.0 - weight) + color_2.r * weight,
        color_1.g * (1.0 - weight) + color_2.g * weight,
        color_1.b * (1.0 - weight) + color_2.b * weight,
        color_1.a * (1.0 - weight) + color_2.a * weight,
    )
}

fn generate_nested_vec<T: Clone>(num_rows: usize, num_columns: usize, default: T) -> Vec<Vec<T>> {
    let mut row = Vec::new();
    row.resize(num_columns, default);
    let mut inner = Vec::new();
    inner.resize(num_rows, row);
    inner
}
