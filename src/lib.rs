pub mod logic {
    pub mod constraints;
    pub mod grid;
    pub mod intersection;
    pub mod pixel_grid;
    pub mod rails;
}
pub mod level_history;
pub mod levels;
pub mod render;
pub mod slider;
pub mod sound;
pub mod theme;

pub mod scenes {
    pub mod main_menu;
    pub use main_menu::main_menu;
    pub mod options;
    pub use options::options;
    pub mod loading_screen;
    pub use loading_screen::loading_screen;
    pub mod level_selector;
    pub use level_selector::level_selector;
    pub mod play {
        pub mod panel;
        pub mod play;
        pub use play::*;
    }
    pub use play::play::play;
}

use crate::logic::constraints::{Constraints, RailCoord};
use crate::logic::grid::{get, Grid};
use crate::theme::Layout;
use juquad::widgets::anchor::Vertical;
use juquad::widgets::{StateStyle, Style};
use macroquad::prelude::*;

pub const STARTING_SECTION: i32 = 0;
pub const STARTING_LEVEL: i32 = 0;

pub const DEFAULT_SHOW_SOLUTION: bool = false;
pub const SEE_SOLUTION_DURING_GAME: bool = true;
pub const VISUALIZE: bool = false;
pub const FONT_SIZE_CHANGING: bool = false;
pub const STEP_GENERATION: bool = false;
pub const SHOW_FPS: bool = false;
pub const CACHE_TEXTURE: bool = true;
pub const SHOW_SLIDER: bool = false;

pub const DEFAULT_VOLUME: f32 = 0.6;
// pub const DEFAULT_VOLUME: f32 = 0.0;
pub const TOOLTIP_DELAY: f64 = 2.5;

pub const TRANSPARENT: Color = Color::new(0.0, 0.0, 0.0, 0.0);
// pub const BACKGROUND: Color = Color::new(0.1, 0.1, 0.1, 1.00);
pub const BACKGROUND: Color = DARKDARKGRAY;
pub const BACKGROUND_2: Color = color_average_weight(BACKGROUND, GRAY, 0.5);
pub const PANEL_BACKGROUND: Color = DISABLED_CELL;
// pub const PANEL_BACKGROUND: Color = LIGHTGRAY;

pub const SUCCESS: Color = color_average_weight(LIGHTGRAY, BLUE, 0.5);
pub const FAILING_DARK: Color = color_average(FAILING, BLACK);
// pub const FAILING: Color = color_average_weight(color_average_weight(BACKGROUND, SUCCESS, 0.3), ORANGE, 0.5);
pub const FAILING: Color = color_average_weight(GRAY, ORANGE, 0.5);
pub const FAILING_TRANSPARENT: Color = color_average_weight(FAILING, TRANSPARENT, 0.2);
// pub const SUCCESS: Color = Color::new(0.10, 0.75, 0.19, 1.00); // less saturated GREEN
pub const FAILING_TRANSPARENT_DARK: Color = color_average_weight(FAILING_DARK, TRANSPARENT, 0.2);
pub const SUCCESS_DARK: Color = color_average(SUCCESS, BLACK);
pub const SUCCESS_TRANSPARENT: Color = color_average_weight(SUCCESS, TRANSPARENT, 0.2);
pub const SUCCESS_LIGHT: Color = color_average_weight(SUCCESS, WHITE, 0.8);

// pub const TRIANGLE: Color = Color::new(0.40, 0.7, 0.9, 1.00); // darker sky blue
// pub const TRIANGLE: Color = color_average_weight(SUCCESS, FAILING, 0.2);
pub const TRIANGLE: Color = color_average_weight(GRAY, LIGHTGRAY, 0.8);
pub const TRIANGLE_BORDER: Color = color_average_weight(BLACK, TRIANGLE, 0.25);
pub const RAIL: Color = TRIANGLE;
pub const RAIL_BORDER: Color = TRIANGLE_BORDER;
pub const UNREACHABLE_RAIL: Color = FAILING;

pub const ENABLED_CELL: Color = color_average_weight(TRIANGLE, DISABLED_CELL, 0.5);
pub const DISABLED_CELL: Color = DARKGRAY;
pub const HOVERED_CELL: Color = color_average(ENABLED_CELL, DISABLED_CELL);
pub const FIX_MARKER: Color = color_average_weight(BLACK, DARKGRAY, 0.3);
pub const USER_FIX_MARKER: Color = LIGHTLIGHTGRAY;

pub const LIGHTLIGHTGRAY: Color = color_average(LIGHTGRAY, WHITE);
pub const LIGTHDARKDARKGRAY: Color = color_average_weight(BLACK, DARKGRAY, 0.8);
pub const DARKDARKGRAY: Color = color_average_weight(BLACK, DARKGRAY, 0.7);

pub const STYLE: Style = Style {
    at_rest: StateStyle {
        bg_color: LIGTHDARKDARKGRAY,
        text_color: LIGHTLIGHTGRAY,
        border_color: LIGHTGRAY,
    },
    hovered: StateStyle {
        bg_color: LIGHTLIGHTGRAY,
        text_color: TRIANGLE_BORDER,
        border_color: TRIANGLE,
    },
    pressed: StateStyle {
        bg_color: GRAY,
        text_color: TRIANGLE_BORDER,
        border_color: TRIANGLE,
    },
};

pub const TEXT_STYLE: StateStyle = StateStyle {
    bg_color: DARKDARKGRAY,
    text_color: LIGHTLIGHTGRAY,
    border_color: GRAY,
};

pub const NUM_ROWS: i32 = 10;
pub const NUM_COLUMNS: i32 = 11;
pub const MAX_CELLS_COEF: f32 = 0.5;
pub const CLUE_PERCENTAGE: u32 = 30;

// pub const BUTTON_PANEL_WIDTH: f32 = 300.0;

const DEFAULT_ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const DEFAULT_WINDOW_WIDTH: i32 = 990;
pub const DEFAULT_WINDOW_HEIGHT: i32 = width_to_height_default(DEFAULT_WINDOW_WIDTH as f32) as i32;
pub const DEFAULT_WINDOW_TITLE: &str = "Metro Loop";

pub type AnyError = Box<dyn std::error::Error>;

pub fn new_layout(screen_width: f32, screen_height: f32) -> Layout {
    const FONT_SIZE: f32 = 16.0;
    const CELL_WIDTH: f32 = 50.0;
    const CELL_HEIGHT: f32 = 50.0;
    const GRID_PAD: f32 = 30.0;
    const CELL_PAD: f32 = 5.0;

    let update_scale = |value: f32| choose_scale(screen_width, screen_height, value);
    let font_size = update_scale(FONT_SIZE);
    let grid_pad = update_scale(GRID_PAD);
    let cell_pad = update_scale(CELL_PAD);

    Layout {
        screen_width,
        screen_height,
        font_size,
        cell_width: CELL_WIDTH,
        cell_height: CELL_HEIGHT,
        grid_pad,
        cell_pad,
        default_rows: NUM_ROWS,
        default_columns: NUM_COLUMNS,
    }
    .readjust()
}

pub enum NextStage {
    MainMenu,
    LevelSelector,
    Campaign,
    Options,
    Quit,
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

const fn choose_scale(width: f32, height: f32, font_size: f32) -> f32 {
    let min_side = width.min(height * 16.0 / 9.0);
    font_size
        * if min_side < 1600.0 {
            1.0
        } else if min_side < 2500.0 {
            1.5
        } else {
            2.0
        }
}

fn generate_nested_vec<T: Clone>(num_rows: usize, num_columns: usize, default: T) -> Vec<Vec<T>> {
    let row = vec![default; num_columns];
    let rows = vec![row; num_rows];
    rows
}
pub const fn width_to_height_default(width: f32) -> f32 {
    width_to_height(width, DEFAULT_ASPECT_RATIO)
}
pub const fn width_to_height(width: f32, aspect_ratio: f32) -> f32 {
    width / aspect_ratio
}
pub const fn height_to_width_default(height: f32) -> f32 {
    width_to_height(height, DEFAULT_ASPECT_RATIO)
}
pub const fn height_to_width(height: f32, aspect_ratio: f32) -> f32 {
    height * aspect_ratio
}
