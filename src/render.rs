use crate::constraints::Satisfaction;
use crate::*;
use juquad::draw::draw_rect;
use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::button::Button;
use juquad::widgets::button_group::LabelGroup;
use juquad::widgets::text::TextRect;
use juquad::widgets::Widget;
use macroquad::math::f32;
use macroquad::prelude::*;

pub fn render_satisfaction(
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
        let show_text = if *show_solution {
            "Hide solution"
        } else {
            "Show solution"
        };
        let mut show = new_button(show_text, show_anchor);
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
    }
}

pub fn render_grid(grid: &Grid, hovered_cell: &Option<(i32, i32)>) {
    for i_row in 0..NUM_ROWS {
        for i_column in 0..NUM_COLUMNS {
            let current_cell = *get(grid, i_row, i_column);

            let color = if current_cell {
                ENABLED_CELL
            } else {
                DISABLED_CELL
            };
            let color = if let Some(hovered) = hovered_cell.clone() {
                if (i_row, i_column) == hovered {
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

                let top_left = cell_top_left(i_row, i_column);
                let second_corner = top_left + vec2(CELL_WIDTH, 0.0);
                draw_line_v(top_left, second_corner, TRIANGLE_BORDER);
                let top_left = top_left - vec2(0.0, CELL_PAD + 1.0);
                let second_corner = second_corner - vec2(0.0, CELL_PAD + 1.0);
                draw_line_v(top_left, second_corner, TRIANGLE_BORDER);

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

                let top_left = cell_top_left(i_row, i_column) + vec2(1.0, 0.0);
                let second_corner = top_left + vec2(0.0, CELL_WIDTH);
                draw_line_v(top_left, second_corner, TRIANGLE_BORDER);
                let top_left = top_left - vec2(CELL_PAD + 1.0, 0.0);
                let second_corner = second_corner - vec2(CELL_PAD + 1.0, 0.0);
                draw_line_v(top_left, second_corner, TRIANGLE_BORDER);

                let intersection = Rect::new(top_left.x, top_left.y, CELL_PAD, CELL_PAD);
                draw_rect(intersection, RAIL);

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
    // intersections
    for i_row in 1..NUM_ROWS {
        for i_column in 1..NUM_COLUMNS {
            let below = grid.rails.get_vert(i_row, i_column);
            let above = grid.rails.get_vert(i_row - 1, i_column);
            let right = grid.rails.get_horiz(i_row, i_column);
            let left = grid.rails.get_horiz(i_row, i_column - 1);

            let current_cell = *get(grid, i_row, i_column);
            let above_cell = *get(grid, i_row - 1, i_column);
            let left_cell = *get(grid, i_row, i_column - 1);
            let left_above_cell = *get(grid, i_row - 1, i_column - 1);

            let bottom_right = cell_top_left(i_row, i_column);
            let top_left = bottom_right - CELL_PAD;
            let _top_right = top_left + vec2(CELL_PAD, 0.0);
            let _bottom_left = top_left + vec2(0.0, CELL_PAD);
            let _center = top_left_rail_intersection(i_row, i_column);
            let intersection = Rect::new(top_left.x, top_left.y, CELL_PAD, CELL_PAD);
            match (below, above, right, left) {
                (Vertical::Center, Vertical::Center, Horizontal::Center, Horizontal::Center) => {}
                _ => draw_rect(intersection, RAIL),
            }
            if current_cell && left_above_cell && !above_cell && !left_cell {
                let start = bottom_right - vec2(CELL_PAD, 0.0);
                let end = bottom_right - vec2(0.0, CELL_PAD);
                draw_line(start.x, start.y, end.x, end.y, 1.0, TRIANGLE_BORDER);
            } else if !current_cell && !left_above_cell && above_cell && left_cell {
                let start = cell_top_left(i_row, i_column);
                let end = start - vec2(CELL_PAD, CELL_PAD);
                draw_line(start.x, start.y, end.x, end.y, 1.0, TRIANGLE_BORDER);
            }
        }
    }
}

pub fn render_constraints(constraints: &Constraints, grid: &Grid) {
    let triangle_half_width = 4.0 * CELL_PAD;
    let small_triangle_half_width = 2.0 * CELL_PAD;
    let thickness = 1.5 * CELL_PAD;
    // let offset = thickness * 0.8;
    enum Constraint {
        Station,
        Blockade,
    }
    for constraint in &constraints.rails {
        let (color, color_border) = if matches_constraint(grid, constraint) {
            (SUCCESS, SUCCESS_DARK)
        } else {
            (FAILING, FAILING_DARK)
        };

        let (row, column, direction, constraint_render) = match *constraint {
            RailCoord::Horizontal {
                row,
                column,
                direction,
            } => match direction {
                Horizontal::Left => (row, column, vec2(-1.0, 0.0), Constraint::Station),
                Horizontal::Center => (row, column, vec2(1.0, 0.0), Constraint::Blockade),
                Horizontal::Right => (row, column, vec2(1.0, 0.0), Constraint::Station),
            },
            RailCoord::Vertical {
                row,
                column,
                direction,
            } => match direction {
                Vertical::Top => (row, column, vec2(0.0, -1.0), Constraint::Station),
                Vertical::Center => (row, column, vec2(0.0, 1.0), Constraint::Blockade),
                Vertical::Bottom => (row, column, vec2(0.0, 1.0), Constraint::Station),
            },
        };

        let corner = top_left_rail_intersection(row, column);
        let reverse = direction.x + direction.y < 0.0;
        let start = corner - reverse as i32 as f32 * direction * (CELL_WIDTH + CELL_PAD);
        let end = start + direction * (CELL_WIDTH + CELL_PAD);
        let mid = (start + end) * 0.5;
        let diff = (end - start).normalize();
        let to_left = vec2(diff.y, -diff.x);

        match constraint_render {
            Constraint::Station => {
                let left = mid + to_left * small_triangle_half_width;
                let right = mid - to_left * small_triangle_half_width;
                let outer_left = mid + to_left * triangle_half_width;
                let outer_right = mid - to_left * triangle_half_width;
                let tip = mid + diff * small_triangle_half_width;
                let outer_tip = mid + diff * triangle_half_width;

                draw_triangle(outer_left, left, outer_tip, color);
                draw_triangle(left, tip, outer_tip, color);
                draw_triangle(right, outer_tip, tip, color);
                draw_triangle(right, outer_right, outer_tip, color);
                draw_lines(
                    &[tip, left, outer_left, outer_tip, outer_right, right, tip],
                    color_border,
                );
            }
            Constraint::Blockade => {
                let forward = diff * thickness * 0.5;
                let leftward = to_left * small_triangle_half_width;
                let a = mid + forward + leftward;
                let b = mid + forward - leftward;
                let c = mid - forward + leftward;
                let d = mid - forward - leftward;
                draw_triangle(a, c, b, color);
                draw_triangle(b, c, d, color);
                draw_lines(&[a, b, d, c, a], color_border);
            }
        }
    }
}

fn top_left_rail_intersection(i_row: i32, i_column: i32) -> Vec2 {
    cell_top_left(i_row, i_column) - CELL_PAD * 0.5
}

fn cell_top_left(i_row: i32, i_column: i32) -> Vec2 {
    let x = GRID_PAD + i_column as f32 * (CELL_WIDTH + CELL_PAD);
    let y = GRID_PAD + i_row as f32 * (CELL_HEIGHT + CELL_PAD);
    vec2(x, y)
}

pub fn new_button(text: &str, anchor: Anchor) -> Button {
    Button::new(text, anchor, FONT_SIZE)
}
pub fn render_button(button: &Button) {
    button.render_default(&STYLE);
}

pub fn draw_bordered_triangle(p_1: Vec2, p_2: Vec2, p_3: Vec2, color: Color, border: Color) {
    draw_triangle(p_1, p_2, p_3, color);
    draw_triangle_lines(p_1, p_2, p_3, 1.0, border);
}

fn render_tick(anchor: Anchor, size: f32) {
    let rect = anchor.get_rect(vec2(size, size));
    draw_rect(rect, SUCCESS_DARK);
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
    draw_rect(rect, FAILING_DARK);
    // draw_rect_lines(rect, 2.0, BLUE);
    let start = rect.point() + rect.size() * 0.25;
    let end = rect.point() + rect.size() * 0.75;
    draw_line(start.x, start.y, end.x, end.y, CELL_PAD, FAILING);
    draw_line(start.x, end.y, end.x, start.y, CELL_PAD, FAILING);
}

pub fn draw_line_v(start: Vec2, end: Vec2, color: Color) {
    draw_line_thickness(start, end, 1.0, color)
}

fn draw_line_thickness(start: Vec2, end: Vec2, thickness: f32, color: Color) {
    draw_line(start.x, start.y, end.x, end.y, thickness, color)
}

pub fn draw_lines(points: &[Vec2], color: Color) {
    assert!(points.len() >= 2);
    for i in 1..points.len() {
        draw_line_v(points[i - 1], points[i], color);
    }
}
