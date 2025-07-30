use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use macroquad::rand::{rand, srand};

pub type Grid = Vec<Vec<bool>>;
pub type Cell = bool;

fn get_mut(grid: &mut Grid, row: usize, column: usize) -> &mut Cell {
    grid.get_mut(row).unwrap().get_mut(column).unwrap()
}

fn get(grid: &Grid, row: usize, column: usize) -> &Cell {
    grid.get(row).unwrap().get(column).unwrap()
}

const SIZE: usize = 10;
const CELL_WIDTH: f32 = 50.0;
const CELL_HEIGHT: f32 = 50.0;
const CELL_PAD: f32 = 5.0;
const GRID_PAD: f32 = 30.0;

const DEFAULT_WINDOW_WIDTH: i32 = ((CELL_WIDTH + CELL_PAD) * SIZE as f32 + GRID_PAD * 2.0) as i32;
const DEFAULT_WINDOW_HEIGHT: i32 = ((CELL_HEIGHT + CELL_PAD) * SIZE as f32 + GRID_PAD * 2.0) as i32;
const DEFAULT_WINDOW_TITLE: &str = "Metro Loop";

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let seed = now() as u64;
    srand(seed);
    let mut grid = reset().await;
    let (_sw, _sh) = (screen_width(), screen_height());
    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.00));
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::R) {
            grid = reset().await;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let pos = Vec2::from(mouse_position());
            let grid_indexes =
                (pos - GRID_PAD + CELL_PAD * 0.5) / (vec2(CELL_WIDTH, CELL_HEIGHT) + CELL_PAD);
            let i_row = grid_indexes.y as usize;
            let i_column = grid_indexes.x as usize;
            if i_column > 0 && i_column < SIZE && i_row > 0 && i_row < SIZE {
                let cell = get_mut(&mut grid, i_row, i_column);
                *cell = !*cell;
            }
            // draw_text(&format!("pos clicked: {:?}", grid_indexes), 0.0, 16.0, 16.0, BLACK);
        }
        render_grid(&mut grid);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //
        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn render_grid(mut grid: &mut Vec<Vec<bool>>) {
    for i_row in 0..SIZE {
        for i_column in 0..SIZE {
            let current_cell = *get_mut(&mut grid, i_row, i_column);
            let color = if current_cell { DARKGREEN } else { DARKGRAY };
            draw_rectangle(
                GRID_PAD + i_column as f32 * (CELL_WIDTH + CELL_PAD),
                GRID_PAD + i_row as f32 * (CELL_HEIGHT + CELL_PAD),
                CELL_WIDTH,
                CELL_HEIGHT,
                color,
            );

            if i_row > 0 && i_column > 0 {
                let left = *get_mut(&mut grid, i_row, i_column - 1);
                let above = *get_mut(&mut grid, i_row - 1, i_column);
                if current_cell != above {
                    let start_x =
                        GRID_PAD - CELL_PAD * 0.5 + i_column as f32 * (CELL_WIDTH + CELL_PAD);
                    let start_y =
                        GRID_PAD - CELL_PAD * 0.5 + i_row as f32 * (CELL_HEIGHT + CELL_PAD);
                    let end_x =
                        GRID_PAD - CELL_PAD * 0.5 + (i_column + 1) as f32 * (CELL_WIDTH + CELL_PAD);
                    let end_y = GRID_PAD - CELL_PAD * 0.5 + i_row as f32 * (CELL_HEIGHT + CELL_PAD);
                    draw_line(start_x, start_y, end_x, end_y, CELL_PAD, GREEN);
                    let mid = (start_x + end_x) * 0.5;
                    let triangle_width = 2.0 * CELL_PAD;
                    draw_bordered_triangle(
                        vec2(mid, start_y - triangle_width),
                        vec2(mid, start_y + triangle_width),
                        vec2(
                            mid + triangle_width * if current_cell { 1.0 } else { -1.0 },
                            start_y,
                        ),
                        ORANGE,
                    );
                }
                if current_cell != left {
                    let start_x =
                        GRID_PAD - CELL_PAD * 0.5 + i_column as f32 * (CELL_WIDTH + CELL_PAD);
                    let start_y =
                        GRID_PAD - CELL_PAD * 0.5 + i_row as f32 * (CELL_HEIGHT + CELL_PAD);
                    let end_x =
                        GRID_PAD - CELL_PAD * 0.5 + i_column as f32 * (CELL_WIDTH + CELL_PAD);
                    let end_y =
                        GRID_PAD - CELL_PAD * 0.5 + (i_row + 1) as f32 * (CELL_HEIGHT + CELL_PAD);
                    draw_line(start_x, start_y, end_x, end_y, CELL_PAD, GREEN);

                    let mid = (start_y + end_y) * 0.5;
                    let triangle_width = 2.0 * CELL_PAD;
                    draw_bordered_triangle(
                        vec2(start_x - triangle_width, mid),
                        vec2(start_x + triangle_width, mid),
                        vec2(
                            start_x,
                            mid - triangle_width * if current_cell { 1.0 } else { -1.0 },
                        ),
                        ORANGE,
                    );
                }
            }
        }
    }
}

async fn reset() -> Vec<Vec<bool>> {
    let mut grid = Vec::new();
    for _ in 0..SIZE {
        grid.push(Vec::from([false; SIZE]))
    }

    let mut enabled = Vec::new();

    *get_mut(&mut grid, SIZE / 2, SIZE / 2) = true;
    enabled.push((SIZE / 2, SIZE / 2));
    let mut i = 0;
    while enabled.len() < 30 {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) || true {
            i += 1;
            if i > 1000 {
                break;
            }

            // let index = rand() as usize % enabled.len();
            let mut index = enabled.len() - 1;
            let (mut row, mut column) = enabled[index];
            let mut neighbours = count_neighbours(&grid, row, column);
            for _ in 0..20 {
                if neighbours > 2 {
                    println!("rejected: ({}, {}), neighbours {}", row, column, neighbours);
                    index = rand() as usize % enabled.len();
                    (row, column) = enabled[index];
                    neighbours = count_neighbours(&grid, row, column);
                }
            }
            if neighbours < 3 {
                let neighbour = rand() as usize % 4;
                println!(
                    "index: {} ({}, {}), neighbour {}",
                    index, row, column, neighbour
                );
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
                        *get_mut(&mut grid, new_row, new_column) = true;
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
        render_grid(&mut grid);
        next_frame().await;
    }
    println!("tried {} iterations", i);
    grid
}

fn count_neighbours(grid: &Grid, row: usize, column: usize) -> i32 {
    *get(grid, row + 1, column) as i32
        + *get(grid, row - 1, column) as i32
        + *get(grid, row, column + 1) as i32
        + *get(grid, row, column - 1) as i32
}

fn in_range(row: usize, column: usize) -> bool {
    row > 0 && row < SIZE - 1 && column > 0 && column < SIZE - 1
}

fn draw_bordered_triangle(p_1: Vec2, p_2: Vec2, p_3: Vec2, color: Color) {
    draw_triangle(p_1, p_2, p_3, color);
    draw_triangle_lines(p_1, p_2, p_3, 1.0, BLACK);
}
