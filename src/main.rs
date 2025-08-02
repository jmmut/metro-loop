use juquad::draw::draw_rect;
use juquad::widgets::anchor::Anchor;
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use macroquad::rand::{rand, srand};
use metro_loop::constraints::{choose_constraints, compute_satisfaction, count_loops, Constraints};
use metro_loop::grid::{count_neighbours, get_mut, in_range, Grid};
use metro_loop::render::{new_button, render_button, render_constraints, render_grid, render_satisfaction};
use metro_loop::{grid_height, grid_width, BACKGROUND, BACKGROUND_2, BUTTON_PANEL_WIDTH, CELL_HEIGHT, CELL_PAD, CELL_WIDTH, DEFAULT_SHOW_SOLUTION, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE, DEFAULT_WINDOW_WIDTH, GRID_PAD, NUM_COLUMNS, NUM_ROWS, PANEL_BACKGROUND, STEP_GENERATION, VISUALIZE};

#[macroquad::main(window_conf)]
async fn main() {
    let seed = now() as u64;
    srand(seed);
    let (mut solution, mut grid, mut constraints, mut show_solution) = reset(VISUALIZE).await;

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
        let mut reset_button = new_button(
            "New Game",
            Anchor::top_center(
                button_panel.x + button_panel.w * 0.5,
                button_panel.y + GRID_PAD,
            ),
        );
        if is_key_pressed(KeyCode::R) || reset_button.interact().is_clicked() {
            (solution, grid, constraints, show_solution) = reset(VISUALIZE).await;
        }
        let pos = Vec2::from(mouse_position());
        let grid_indexes =
            (pos - GRID_PAD + CELL_PAD * 0.5) / (vec2(CELL_WIDTH, CELL_HEIGHT) + CELL_PAD);
        let i_row = grid_indexes.y as i32;
        let i_column = grid_indexes.x as i32;
        let hovered_cell = if in_range(i_row, i_column) {
            Some((i_row, i_column))
        } else {
            None
        };
        // draw_text(&format!("pos clicked: {:?}", grid_indexes), 0.0, 16.0, 16.0, BLACK);
        if is_mouse_button_pressed(MouseButton::Left) && !show_solution {
            if let Some((i_row, i_column)) = hovered_cell.clone() {
                let cell = get_mut(&mut grid, i_row, i_column);
                *cell = !*cell;
                grid.recalculate_rails();
            }
        }

        let satisfaction = compute_satisfaction(&grid, &constraints);

        draw_rect(button_panel, PANEL_BACKGROUND);
        render_button(&reset_button);
        render_satisfaction(
            &satisfaction,
            reset_button.rect(),
            button_panel,
            &mut show_solution,
        );
        if show_solution {
            render_grid(&solution, &hovered_cell);
        } else {
            render_grid(&grid, &hovered_cell);
            render_constraints(&constraints, &grid);
        }

        next_frame().await
    }
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

async fn reset(visualize: bool) -> (Grid, Grid, Constraints, bool) {
    let mut solution = generate_grid(visualize).await;
    while count_loops(&solution) != 1 {
        solution = generate_grid(visualize).await;
    }
    // println!("tried {} iterations", i);
    solution.recalculate_rails();
    let mut grid = Grid::new(NUM_ROWS, NUM_COLUMNS, solution.root);
    grid.recalculate_rails();
    let constraints = choose_constraints(&solution);
    let show_solution = DEFAULT_SHOW_SOLUTION;
    (solution, grid, constraints, show_solution)
}

async fn generate_grid(visualize: bool) -> Grid {
    let mut solution = Grid::new(NUM_ROWS, NUM_COLUMNS, ivec2(NUM_ROWS / 2, NUM_COLUMNS / 2));
    let mut enabled = Vec::new();

    enabled.push((solution.root.y, solution.root.x));
    let mut i = 0;
    while enabled.len() < 30 {
        if visualize && is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) || !STEP_GENERATION {
            i += 1;
            if i > 1000 {
                break;
            }

            // let index = rand() % enabled.len();
            let mut index = enabled.len() - 1;
            let (mut row, mut column) = enabled[index];
            let mut neighbours = count_neighbours(&solution, row, column);
            let mut low_neighbours_attempts = 0;
            while neighbours > 2 {
                low_neighbours_attempts += 1;
                if low_neighbours_attempts > 20 {
                    break;
                }
                // println!("rejected: ({}, {}), neighbours {}", row, column, neighbours);
                index = rand() as usize % enabled.len();
                (row, column) = enabled[index];
                neighbours = count_neighbours(&solution, row, column);
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
                    let above_root = (solution.root.y - 1, solution.root.x);
                    // if the chosen neighbour is already enabled, choose another neighbour
                    if in_range(new_row, new_column) && (new_row, new_column) != above_root {
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
    solution
}
