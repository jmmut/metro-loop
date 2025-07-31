mod rails;

use crate::rails::{count_neighbours, get_mut, in_range, Grid, RailCoord};
use juquad::widgets::anchor::{Anchor, Horizontal, Vertical};
use juquad::widgets::button::Button;
use juquad::widgets::{StateStyle, Style};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use macroquad::rand::{rand, srand};

const BACKGROUND: Color = Color::new(0.1, 0.1, 0.1, 1.00);
const BACKGROUND_2: Color = Color::new(0.05, 0.05, 0.05, 1.00);
const TRIANGLE: Color = ORANGE;
const TRIANGLE_BORDER: Color = Color::new(0.5, 0.3, 0.00, 1.00);
const RAIL: Color = GREEN;
const ENABLED_CELL: Color = DARKGREEN;
const DISABLED_CELL: Color = DARKGRAY;
const HOVERED_CELL: Color = Color::new(0.15, 0.38, 0.22, 1.0);

const STYLE: Style = Style {
    at_rest: StateStyle {
        bg_color: LIGHTGRAY,
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

const FONT_SIZE: f32 = 16.0;

const NUM_ROWS: i32 = 11;
const NUM_COLUMNS: i32 = 11;
const CELL_WIDTH: f32 = 50.0;
const CELL_HEIGHT: f32 = 50.0;
const CELL_PAD: f32 = 5.0;
const GRID_PAD: f32 = 30.0;
const BUTTON_PANEL_WIDTH: f32 = 300.0;

const DEFAULT_WINDOW_WIDTH: i32 = (grid_width() + BUTTON_PANEL_WIDTH + 3.0 * GRID_PAD) as i32;
const DEFAULT_WINDOW_HEIGHT: i32 = (grid_height() + 2.0 * GRID_PAD) as i32;
const DEFAULT_WINDOW_TITLE: &str = "Metro Loop";

#[macroquad::main(window_conf)]
async fn main() {
    let seed = now() as u64;
    srand(seed);
    let (mut _solution, mut grid) = reset(false).await;
    // let constraints = choose_constraints(&solution);

    let (_sw, _sh) = (screen_width(), screen_height());
    let button_panel = Rect::new(
        grid_width() + GRID_PAD * 2.0,
        GRID_PAD,
        BUTTON_PANEL_WIDTH,
        grid_height(),
    );
    loop {
        clear_background(BACKGROUND);
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        let mut reset_button =
            new_button("Reset", Anchor::top_left(button_panel.x, button_panel.y));
        if is_key_pressed(KeyCode::R) || reset_button.interact().is_clicked() {
            (_solution, grid) = reset(false).await;
        }
        let pos = Vec2::from(mouse_position());
        let grid_indexes =
            (pos - GRID_PAD + CELL_PAD * 0.5) / (vec2(CELL_WIDTH, CELL_HEIGHT) + CELL_PAD);
        let i_row = grid_indexes.y as i32;
        let i_column = grid_indexes.x as i32;
        let hovered_cell = if i_column > 0 && i_column < NUM_COLUMNS - 1 && i_row > 0 && i_row < NUM_ROWS - 1 {
            Some((i_row, i_column))
        } else {
            None
        };
        // draw_text(&format!("pos clicked: {:?}", grid_indexes), 0.0, 16.0, 16.0, BLACK);
        if is_mouse_button_pressed(MouseButton::Left) {
            if let Some((i_row, i_column)) = hovered_cell.clone() {
                let cell = get_mut(&mut grid, i_row, i_column);
                *cell = !*cell;
                grid.recalculate_rails();
            }
        }
        render_grid(&mut grid, &hovered_cell);
        // render_constraints(&constraints);
        render_button(&reset_button);
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //
        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn render_constraints(constraints: &Vec<RailCoord>) {
    todo!()
}

fn new_button(text: &str, anchor: Anchor) -> Button {
    Button::new(text, anchor, FONT_SIZE)
}
fn render_button(button: &Button) {
    button.render_default(&STYLE);
}
const fn grid_width() -> f32 {
    (CELL_WIDTH + CELL_PAD) * NUM_COLUMNS as f32 - CELL_PAD
}
const fn grid_height() -> f32 {
    (CELL_HEIGHT + CELL_PAD) * NUM_ROWS as f32 - CELL_PAD
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}

fn render_grid(mut grid: &mut Grid, hovered_cell: &Option<(i32, i32)>) {
    for i_row in 0..NUM_ROWS {
        for i_column in 0..NUM_COLUMNS {
            let current_cell = *get_mut(&mut grid, i_row, i_column);

            let color = if current_cell {
                ENABLED_CELL
            } else {
                DISABLED_CELL
            };
            let color = if let Some((hovered_row, hovered_column)) = hovered_cell.clone() {
                if i_row == hovered_row && i_column == hovered_column {
                    // BLUE
                    HOVERED_CELL
                } else {
                    color
                }
            } else {
                color
            };
            draw_rectangle(
                GRID_PAD + i_column as f32 * (CELL_WIDTH + CELL_PAD),
                GRID_PAD + i_row as f32 * (CELL_HEIGHT + CELL_PAD),
                CELL_WIDTH,
                CELL_HEIGHT,
                color,
            );
        }
    }

    for i_row in 1..grid.rails.horiz_rows() -1 {
        for i_column in 1..grid.rails.horiz_columns() -1 {
            let start_x = GRID_PAD - CELL_PAD * 0.5 + i_column as f32 * (CELL_WIDTH + CELL_PAD);
            let y = GRID_PAD - CELL_PAD * 0.5 + i_row as f32 * (CELL_HEIGHT + CELL_PAD);
            let end_x = GRID_PAD - CELL_PAD * 0.5 + (i_column + 1) as f32 * (CELL_WIDTH + CELL_PAD);
            let direction = grid.rails.get_horiz(i_row, i_column);
            if direction != Horizontal::Center {
                draw_line(start_x, y, end_x, y, CELL_PAD, RAIL);
                let sign = match direction {
                    Horizontal::Left => -1.0,
                    Horizontal::Center => 0.0,
                    Horizontal::Right => 1.0,
                };
                let mid = (start_x + end_x) * 0.5;
                let triangle_width = 2.0 * CELL_PAD;
                let above = vec2(mid, y - triangle_width);
                let below = vec2(mid, y + triangle_width);
                let tip = vec2(mid + triangle_width * sign, y);
                draw_bordered_triangle(above, below, tip, TRIANGLE, TRIANGLE_BORDER);
            }
        }
    }
    for i_row in 1..grid.rails.vert_rows() -1 {
        for i_column in 1..grid.rails.vert_columns() -1 {
            let x = GRID_PAD - CELL_PAD * 0.5 + i_column as f32 * (CELL_WIDTH + CELL_PAD);
            let start_y = GRID_PAD - CELL_PAD * 0.5 + i_row as f32 * (CELL_HEIGHT + CELL_PAD);
            let end_y = GRID_PAD - CELL_PAD * 0.5 + (i_row + 1) as f32 * (CELL_HEIGHT + CELL_PAD);
            let direction = grid.rails.get_vert(i_row, i_column);

            if direction != Vertical::Center {
                draw_line(x, start_y, x, end_y, CELL_PAD, RAIL);
                let sign = match direction {
                    Vertical::Top => -1.0,
                    Vertical::Center => 0.0,
                    Vertical::Bottom => 1.0,
                };
                let mid = (start_y + end_y) * 0.5;
                let triangle_width = 2.0 * CELL_PAD;
                let left = vec2(x - triangle_width, mid);
                let right = vec2(x + triangle_width, mid);
                let tip = vec2(x, mid + triangle_width * sign);
                draw_bordered_triangle(left, right, tip, TRIANGLE, TRIANGLE_BORDER);
            }
        }
    }
    // for i_row in 1..grid.rails.vert_rows() -1 {
    //     for i_column in 1..grid.rails.vert_columns() -1 {
    //             if current_cell != left {
    //                 let start_x =
    //                     GRID_PAD - CELL_PAD * 0.5 + i_column as f32 * (CELL_WIDTH + CELL_PAD);
    //                 let start_y =
    //                     GRID_PAD - CELL_PAD * 0.5 + i_row as f32 * (CELL_HEIGHT + CELL_PAD);
    //                 let end_x =
    //                     GRID_PAD - CELL_PAD * 0.5 + i_column as f32 * (CELL_WIDTH + CELL_PAD);
    //                 let end_y =
    //                     GRID_PAD - CELL_PAD * 0.5 + (i_row + 1) as f32 * (CELL_HEIGHT + CELL_PAD);
    //                 draw_line(start_x, start_y, end_x, end_y, CELL_PAD, RAIL);
    //
    //                 let mid = (start_y + end_y) * 0.5;
    //                 let triangle_width = 2.0 * CELL_PAD;
    //                 draw_bordered_triangle(
    //                     vec2(start_x - triangle_width, mid),
    //                     vec2(start_x + triangle_width, mid),
    //                     vec2(
    //                         start_x,
    //                         mid - triangle_width * if current_cell { 1.0 } else { -1.0 },
    //                     ),
    //                     TRIANGLE,
    //                     TRIANGLE_BORDER,
    //                 );
    //             }
    //         }
    //     }
    // }
}

async fn reset(visualize: bool) -> (Grid, Grid) {
    let mut solution = Grid::new(NUM_ROWS, NUM_COLUMNS, ivec2(NUM_ROWS / 2, NUM_COLUMNS / 2));
    let mut enabled = Vec::new();

    enabled.push((solution.root.y, solution.root.x));
    let mut i = 0;
    while enabled.len() < 30 {
        if visualize && is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) || !visualize {
            i += 1;
            if i > 1000 {
                break;
            }

            // let index = rand() % enabled.len();
            let mut index = enabled.len() - 1;
            let (mut row, mut column) = enabled[index];
            let mut neighbours = count_neighbours(&solution, row, column);
            for _ in 0..20 {
                if neighbours > 2 {
                    // println!("rejected: ({}, {}), neighbours {}", row, column, neighbours);
                    index = rand() as usize % enabled.len();
                    (row, column) = enabled[index];
                    neighbours = count_neighbours(&solution, row, column);
                }
            }
            if neighbours < 3 {
                let neighbour = rand() % 4;
                // println!(
                //     "index: {} ({}, {}), neighbour {}",
                //     index, row, column, neighbour
                // );
                if row > 0 && column > 0 {
                    let (new_row, new_column) = match neighbour {
                        0 => (row - 1, column),
                        1 => (row, column + 1),
                        2 => (row + 1, column),
                        3 => (row, column - 1),
                        _ => panic!(),
                    };
                    // if the chosen neighbour is already enabled, choose another neighbour
                    if in_range(new_row, new_column) {
                        *get_mut(&mut solution, new_row, new_column) = true;
                        enabled.push((new_row, new_column));
                    }
                }
            }
            // for i_row in 0..SIZE {
            //     for i_column in 0..SIZE {
            //         let clicked = *get_mut(&mut grid, i_row, i_column);
            //         let char = if clicked { "0" } else { "-" };
            //         print!("{}", char)
            //     }
            //     println!()
            // }
            // println!()
        }
        if visualize {
            clear_background(BACKGROUND_2);
            render_grid(&mut solution, &None);
            next_frame().await;
        }
    }
    // println!("tried {} iterations", i);
    solution.recalculate_rails();
    let mut grid = Grid::new(NUM_ROWS, NUM_COLUMNS, solution.root);
    grid.recalculate_rails();
    (solution, grid)
}

fn draw_bordered_triangle(p_1: Vec2, p_2: Vec2, p_3: Vec2, color: Color, border: Color) {
    draw_triangle(p_1, p_2, p_3, color);
    draw_triangle_lines(p_1, p_2, p_3, 2.0, border);
}
