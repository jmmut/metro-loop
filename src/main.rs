use macroquad::prelude::*;

pub type Grid = Vec<Vec<bool>>;
pub type Cell = bool;

fn get_mut(grid: &mut Grid, row: usize, column: usize) -> &mut Cell {
    grid.get_mut(row).unwrap().get_mut(column).unwrap()
}

const SIZE: usize = 10;
const CELL_WIDTH: f32 = 50.0;
const CELL_HEIGHT: f32 = 50.0;
const CELL_PAD: f32 = 5.0;
const GRID_PAD: f32 = 30.0;

#[macroquad::main("metro-loop")]
async fn main() {
    let mut grid = Vec::new();
    for _ in 0..SIZE {
        grid.push(Vec::from([false; SIZE]))
    }

    *get_mut(&mut grid, 1, 2) = true;

    for i_row in 0..SIZE {
        for i_column in 0..SIZE {
            let clicked = *get_mut(&mut grid, i_row, i_column);
            let char = if clicked { "0" } else { "-" };
            print!("{}", char)
        }
        println!()
    }

    loop {
        clear_background(LIGHTGRAY);
        if is_key_pressed(KeyCode::Escape) {
            break;
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
                        let end_x = GRID_PAD - CELL_PAD * 0.5
                            + (i_column + 1) as f32 * (CELL_WIDTH + CELL_PAD);
                        let end_y =
                            GRID_PAD - CELL_PAD * 0.5 + i_row as f32 * (CELL_HEIGHT + CELL_PAD);
                        draw_line(start_x, start_y, end_x, end_y, CELL_PAD, GREEN);
                    }
                    if current_cell != left {
                        let start_x =
                            GRID_PAD - CELL_PAD * 0.5 + i_column as f32 * (CELL_WIDTH + CELL_PAD);
                        let start_y =
                            GRID_PAD - CELL_PAD * 0.5 + i_row as f32 * (CELL_HEIGHT + CELL_PAD);
                        let end_x =
                            GRID_PAD - CELL_PAD * 0.5 + i_column as f32 * (CELL_WIDTH + CELL_PAD);
                        let end_y = GRID_PAD - CELL_PAD * 0.5
                            + (i_row + 1) as f32 * (CELL_HEIGHT + CELL_PAD);
                        draw_line(start_x, start_y, end_x, end_y, CELL_PAD, GREEN);
                    }
                }
            }
            println!()
        }

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //
        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
