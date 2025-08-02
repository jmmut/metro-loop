use juquad::draw::draw_rect;
use juquad::widgets::Widget;
mod constraints;
mod rails;

use crate::constraints::{
    choose_constraints, compute_satisfaction, count_loops, matches_constraint, Constraints,
    RailCoord, Satisfaction,
};
use crate::rails::{count_neighbours, get, get_mut, in_range, Grid};
use juquad::widgets::anchor::{Anchor, Horizontal, Vertical};
use juquad::widgets::button::Button;
use juquad::widgets::button_group::LabelGroup;
use juquad::widgets::text::TextRect;
use juquad::widgets::{StateStyle, Style};
use macroquad::miniquad::date::now;
use macroquad::prelude::*;
use macroquad::rand::{rand, srand};

const DEFAULT_SHOW_SOLUTION: bool = false;
const VISUALIZE: bool = false;
const STEP_GENERATION: bool = false;

const BACKGROUND: Color = Color::new(0.1, 0.1, 0.1, 1.00);
const BACKGROUND_2: Color = Color::new(0.05, 0.05, 0.05, 1.00);
const TRIANGLE: Color = Color::new(0.40, 0.7, 0.9, 1.00); // darker sky blue
const TRIANGLE_BORDER: Color = color_average_weight(BLACK, BLUE, 0.25);
const RAIL: Color = SKYBLUE;

const FAILING: Color = ORANGE;
const SUCCESS: Color = Color::new(0.10, 0.75, 0.19, 1.00); // less saturated GREEN

const ENABLED_CELL: Color = BLUE;
const DISABLED_CELL: Color = DARKGRAY;
const HOVERED_CELL: Color = color_average(ENABLED_CELL, DISABLED_CELL);

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

        draw_rect(button_panel, GRAY);
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
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //
        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn render_satisfaction(
    satisfaction: &Satisfaction,
    previous_rect: Rect,
    panel: Rect,
    show_solution: &mut bool,
) {
    if satisfaction.success() {
        let anchor = Anchor::below(previous_rect, Horizontal::Center, 30.0);
        let text = TextRect::new(&"SOLVED!", anchor, FONT_SIZE * 2.0);
        text.render_default(&STYLE.at_rest);
        let show_anchor = Anchor::below(text.rect(), Horizontal::Center, 10.0);
        let mut show = new_button("Show solution", show_anchor);
        if show.interact().is_clicked() {
            *show_solution = !*show_solution;
        }
        render_button(&show);
    } else {
        let font_size = FONT_SIZE * 1.25;
        let x = panel.x + font_size * 1.5;
        let y = previous_rect.bottom() + 30.0;
        let anchor = Anchor::top_left(x, y);
        let labels = LabelGroup {
            font_size,
            anchor,
            alignment: Horizontal::Left,
            font: None,
        };
        let text_rects = labels.create([
            &format!("{} incorrect rails", satisfaction.failing_rails),
            &format!("{} cells to activate", satisfaction.cell_diff),
            &format!("{} unconnected loops", satisfaction.unconnected_loops),
        ]);

        for mut text_rect in text_rects {
            let icon_size = text_rect.rect().h;
            text_rect.rect_mut().x += icon_size;
            let anchor = Anchor::top_right_v(text_rect.rect().point());
            (if text_rect.text.chars().next().unwrap() == '0' {
                render_tick
            } else {
                render_cross
            })(anchor, icon_size);
            text_rect.render_default(&STYLE.at_rest);
        }
        // let rails = TextRect::new(&format!("{} incorrect rails", failures), anchor, FONT_SIZE);
        // rails.render_default(&STYLE.at_rest);
        // let anchor = Anchor::below(rails.rect(), Horizontal::Left, FONT_SIZE);
        // let cells = TextRect::new(&format!("{} cells to activate", failures), anchor, FONT_SIZE);
        // cells.render_default(&STYLE.at_rest);
        // let anchor = Anchor::below(cells.rect(), Horizontal::Left, FONT_SIZE);
    }
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

fn render_grid(grid: &Grid, hovered_cell: &Option<(i32, i32)>) {
    for i_row in 0..NUM_ROWS {
        for i_column in 0..NUM_COLUMNS {
            let current_cell = *get(grid, i_row, i_column);

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
            let cell_pos = cell_top_left(i_row, i_column);
            draw_rectangle(cell_pos.x, cell_pos.y, CELL_WIDTH, CELL_HEIGHT, color);
        }
    }

    for i_row in 1..grid.rails.horiz_rows() - 1 {
        for i_column in 1..grid.rails.horiz_columns() - 1 {
            let direction = grid.rails.get_horiz(i_row, i_column);
            if direction != Horizontal::Center {
                let start = top_left_rail_intersection(i_row, i_column);
                let end = top_left_rail_intersection(i_row, i_column + 1);
                draw_line(start.x, start.y, end.x, end.y, CELL_PAD, RAIL);
                let sign = match direction {
                    Horizontal::Left => -1.0,
                    Horizontal::Center => 0.0,
                    Horizontal::Right => 1.0,
                };
                let mid = (start.x + end.x) * 0.5;
                let triangle_width = 2.0 * CELL_PAD;
                let above = vec2(mid, start.y - triangle_width);
                let below = vec2(mid, start.y + triangle_width);
                let tip = vec2(mid + triangle_width * sign, start.y);
                draw_bordered_triangle(above, below, tip, TRIANGLE, TRIANGLE_BORDER);
            }
        }
    }
    for i_row in 1..grid.rails.vert_rows() - 1 {
        for i_column in 1..grid.rails.vert_columns() - 1 {
            let direction = grid.rails.get_vert(i_row, i_column);
            if direction != Vertical::Center {
                let start = top_left_rail_intersection(i_row, i_column);
                let end = top_left_rail_intersection(i_row + 1, i_column);
                draw_line(start.x, start.y, end.x, end.y, CELL_PAD, RAIL);
                let sign = match direction {
                    Vertical::Top => -1.0,
                    Vertical::Center => 0.0,
                    Vertical::Bottom => 1.0,
                };
                let mid = (start.y + end.y) * 0.5;
                let triangle_width = 2.0 * CELL_PAD;
                let left = vec2(start.x - triangle_width, mid);
                let right = vec2(start.x + triangle_width, mid);
                let tip = vec2(start.x, mid + triangle_width * sign);
                draw_bordered_triangle(left, right, tip, TRIANGLE, TRIANGLE_BORDER);
            }
        }
    }
}

fn render_constraints(constraints: &Constraints, grid: &Grid) {
    let triangle_width = 4.0 * CELL_PAD;
    for constraint in &constraints.rails {
        let color = if matches_constraint(grid, constraint) {
            SUCCESS
        } else {
            FAILING
        };
        match *constraint {
            RailCoord::Horizontal {
                row,
                column,
                direction,
            } => {
                let start_corner = top_left_rail_intersection(row, column);
                let end_corner = top_left_rail_intersection(row, column + 1);
                let mid = (start_corner + end_corner) * 0.5;
                match direction {
                    Horizontal::Left => {
                        let (back, tip) = (CELL_PAD, -triangle_width);
                        render_constraint_horiz(triangle_width, mid, back, tip, color);
                    }
                    Horizontal::Center => {
                        let sideways = vec2(0.0, CELL_PAD * 2.0);
                        let start = mid - sideways;
                        let end = mid + sideways;
                        draw_line(start.x, start.y, end.x, end.y, CELL_PAD, color);
                    }
                    Horizontal::Right => {
                        let (back, tip) = (-CELL_PAD, triangle_width);
                        render_constraint_horiz(triangle_width, mid, back, tip, color);
                    }
                };
            }
            RailCoord::Vertical {
                row,
                column,
                direction,
            } => {
                let start_corner = top_left_rail_intersection(row, column);
                let end_corner = top_left_rail_intersection(row + 1, column);
                let mid = (start_corner + end_corner) * 0.5;
                match direction {
                    Vertical::Top => {
                        let (back, tip) = (CELL_PAD, -triangle_width);
                        render_constraint_vert(triangle_width, mid, back, tip, color);
                    }
                    Vertical::Center => {
                        let sideways = vec2(CELL_PAD * 2.0, 0.0);
                        let start = mid - sideways;
                        let end = mid + sideways;
                        draw_line(start.x, start.y, end.x, end.y, CELL_PAD, color);
                    }
                    Vertical::Bottom => {
                        let (back, tip) = (-CELL_PAD, triangle_width);
                        render_constraint_vert(triangle_width, mid, back, tip, color);
                    }
                };
            }
        }
    }
}

fn render_constraint_horiz(triangle_width: f32, mid: Vec2, back: f32, tip: f32, color: Color) {
    let above = mid + vec2(back, triangle_width);
    let below = mid + vec2(back, -triangle_width);
    let tip = mid + vec2(tip, 0.0);
    draw_triangle_lines(above, below, tip, CELL_PAD, color);
}

fn render_constraint_vert(triangle_width: f32, mid: Vec2, back: f32, tip: f32, color: Color) {
    let left = mid + vec2(-triangle_width, back);
    let right = mid + vec2(triangle_width, back);
    let tip = mid + vec2(0.0, tip);
    draw_triangle_lines(left, right, tip, CELL_PAD, color);
}

fn top_left_rail_intersection(i_row: i32, i_column: i32) -> Vec2 {
    cell_top_left(i_row, i_column) - CELL_PAD * 0.5
}

fn cell_top_left(i_row: i32, i_column: i32) -> Vec2 {
    let x = GRID_PAD + i_column as f32 * (CELL_WIDTH + CELL_PAD);
    let y = GRID_PAD + i_row as f32 * (CELL_HEIGHT + CELL_PAD);
    vec2(x, y)
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

fn draw_bordered_triangle(p_1: Vec2, p_2: Vec2, p_3: Vec2, color: Color, border: Color) {
    draw_triangle(p_1, p_2, p_3, color);
    draw_triangle_lines(p_1, p_2, p_3, 1.0, border);
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
fn render_tick(anchor: Anchor, size: f32) {
    let rect = anchor.get_rect(vec2(size, size));
    // draw_rect_lines(rect, 2.0, BLUE);
    let start = rect.point() + rect.size() * 0.25;
    let mid = rect.point() + rect.size() * 0.5;
    let end = rect.point() + rect.size() * 0.75;
    draw_line(start.x, mid.y, mid.x, end.y, CELL_PAD, SUCCESS);
    draw_line(
        mid.x - CELL_PAD * 0.5,
        end.y,
        end.x,
        start.y,
        CELL_PAD,
        SUCCESS,
    );
}
fn render_cross(anchor: Anchor, font_size: f32) {
    let rect = anchor.get_rect(vec2(font_size, font_size));
    // draw_rect_lines(rect, 2.0, BLUE);
    let start = rect.point() + rect.size() * 0.25;
    let end = rect.point() + rect.size() * 0.75;
    draw_line(start.x, start.y, end.x, end.y, CELL_PAD, FAILING);
    draw_line(start.x, end.y, end.x, start.y, CELL_PAD, FAILING);
}
