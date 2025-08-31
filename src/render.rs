use crate::logic::constraints::{matches_constraint_and_reachable, Satisfaction};
use crate::logic::grid::get_cell;
use crate::logic::intersection::{Crossing, Intersection};
use crate::theme::{new_button, new_text, render_text, Theme};
use crate::*;
use juquad::draw::draw_rect;
use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::button::Button;
use juquad::widgets::button_group::LabelGroup;
use juquad::widgets::Widget;
use macroquad::math::f32;
use macroquad::prelude::*;

pub fn render_satisfaction(
    satisfaction: &Satisfaction,
    previous_rect: Rect,
    panel: Rect,
    theme: &Theme,
    show_solution: &mut bool,
) -> Option<Button> {
    let solved = satisfaction.success();
    let mut rect = if solved {
        let anchor = Anchor::below(previous_rect, Horizontal::Center, 30.0);
        let text = new_text(&"SOLVED!", anchor, 2.0, &theme);
        render_text(&text, &STYLE.at_rest);
        text.rect()
    } else {
        let font_size = theme.layout.font_size() * 1.0;
        let anchor = Anchor::below(previous_rect, Horizontal::Center, 30.0);
        let labels = LabelGroup {
            font_size,
            anchor,
            alignment: Horizontal::Left,
            font: Some(theme.resources.font),
        };
        let text_rects = labels.create([
            &format!("{} incorrect rails", satisfaction.failing_rails),
            &format!("{} cells to activate", satisfaction.cell_diff),
            &format!("{} unreachable rails", satisfaction.unreachable_rails),
        ]);
        let mut rect = Rect::default();
        for text_rect in text_rects {
            let icon_size = text_rect.rect().h;
            let anchor = Anchor::top_right_v(text_rect.rect().point());
            (if text_rect.text.chars().next().unwrap() == '0' {
                render_tick
            } else {
                render_cross
            })(anchor, icon_size);
            render_text(&text_rect, &STYLE.at_rest);
            rect = text_rect.rect()
        }
        rect
    };
    if solved || SEE_SOLUTION_DURING_GAME {
        rect.x = panel.x;
        rect.w = panel.w;
        let show_anchor = Anchor::below(rect, Horizontal::Center, 30.0);
        let show_text = if *show_solution {
            "Hide solution"
        } else {
            "Show possible solution"
        };
        let show = new_button(show_text, show_anchor, &theme);
        Some(show)
    } else {
        None
    }
}

pub fn render_cells(grid: &Grid, hovered_cell: &Option<(i32, i32)>, theme: &Theme) {
    for i_row in 0..NUM_ROWS {
        for i_column in 0..NUM_COLUMNS {
            let current_cell = *get_cell(grid, i_row, i_column);

            let color = if current_cell {
                ENABLED_CELL
            } else {
                DISABLED_CELL
            };
            let color = if let Some(hovered) = hovered_cell.clone() {
                let hovered_v = ivec2(hovered.1, hovered.0);
                if (i_row, i_column) == hovered
                    && hovered_v != grid.root
                    && hovered_v != grid.root - ivec2(0, 1)
                {
                    HOVERED_CELL
                } else {
                    color
                }
            } else {
                color
            };
            let cell_pos = cell_top_left(i_row, i_column, theme);
            draw_rectangle(
                cell_pos.x,
                cell_pos.y,
                theme.layout.cell_width(),
                theme.layout.cell_height(),
                color,
            );
        }
    }
}
pub fn render_grid(grid: &Grid, theme: &Theme) {
    for i_row in 0..NUM_ROWS {
        for i_column in 0..NUM_COLUMNS {
            let current_cell = *get(&grid.fixed_cells, i_row, i_column);
            if current_cell {
                let mut intersection = top_left_rail_intersection(i_row, i_column, theme);
                intersection += vec2(theme.layout.cell_width(), theme.layout.cell_height()) * 0.5;
                draw_rect(
                    Rect::new(intersection.x, intersection.y, CELL_PAD, CELL_PAD),
                    TRIANGLE_BORDER,
                );
            }
        }
    }
    // horizontal rails
    for i_row in 1..grid.rails.horiz_rows() - 1 {
        for i_column in 1..grid.rails.horiz_columns() - 1 {
            let reachable = grid.reachable_rails.get_horiz(i_row, i_column);
            let color = if reachable { RAIL } else { UNREACHABLE_RAIL };
            let direction = grid.rails.get_horiz(i_row, i_column);
            if direction != Horizontal::Center {
                let start = top_left_rail_intersection(i_row, i_column, theme);
                let end = top_left_rail_intersection(i_row, i_column + 1, theme);
                draw_line(start.x, start.y, end.x, end.y, CELL_PAD, color);

                let top_left = cell_top_left(i_row, i_column, theme) + vec2(0.0, 1.0);
                let second_corner = top_left + vec2(theme.layout.cell_width(), 0.0);
                draw_line_v(top_left, second_corner, TRIANGLE_BORDER);
                let top_left = top_left - vec2(0.0, CELL_PAD + 1.0);
                let second_corner = second_corner - vec2(0.0, CELL_PAD + 1.0);
                draw_line_v(top_left, second_corner, TRIANGLE_BORDER);
                if reachable {
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
                    draw_bordered_triangle(above, below, tip, color, TRIANGLE_BORDER);
                }
            }
        }
    }

    // vertical rails
    for i_row in 1..grid.rails.vert_rows() - 1 {
        for i_column in 1..grid.rails.vert_columns() - 1 {
            let reachable = grid.reachable_rails.get_vert(i_row, i_column);
            let color = if reachable { RAIL } else { UNREACHABLE_RAIL };
            let direction = grid.rails.get_vert(i_row, i_column);
            if direction != Vertical::Center {
                let start = top_left_rail_intersection(i_row, i_column, theme);
                let end = top_left_rail_intersection(i_row + 1, i_column, theme);
                draw_line(start.x, start.y, end.x, end.y, CELL_PAD, color);

                let top_left = cell_top_left(i_row, i_column, theme) + vec2(0.0, 0.0);
                let second_corner = top_left + vec2(0.0, theme.layout.cell_width());
                draw_line_v(top_left, second_corner, TRIANGLE_BORDER);
                let top_left = top_left - vec2(CELL_PAD + 1.0, 0.0);
                let second_corner = second_corner - vec2(CELL_PAD + 1.0, 0.0);
                draw_line_v(top_left, second_corner, TRIANGLE_BORDER);
                if reachable {
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
                    draw_bordered_triangle(left, right, tip, color, TRIANGLE_BORDER);
                }
            }
        }
    }
    // intersections
    for i_row in 1..grid.intersections.rows() - 1 {
        for i_column in 1..grid.intersections.columns() - 1 {
            let Intersection {
                // right,
                // left,
                // below,
                // above,
                crossing,
            } = grid.intersections.get(i_row, i_column);
            let color = if grid.reachable_rails.get_horiz(i_row, i_column)
                || grid.reachable_rails.get_vert(i_row, i_column)
                || grid.reachable_rails.get_vert(i_row - 1, i_column)
                || grid.reachable_rails.get_horiz(i_row, i_column - 1)
            {
                RAIL
            } else {
                UNREACHABLE_RAIL
            };
            let bottom_right = cell_top_left(i_row, i_column, theme);
            let top_left = bottom_right - CELL_PAD;
            let top_right = top_left + vec2(CELL_PAD, 0.0);
            let bottom_left = top_left + vec2(-1.0, CELL_PAD + 1.0);
            let intersection_rect = Rect::new(top_left.x, top_left.y, CELL_PAD, CELL_PAD);
            let bottom_right = bottom_right + vec2(0.0, 1.0);
            let top_left = top_left + vec2(-1.0, 0.0);
            match crossing {
                Crossing::None => {}
                Crossing::Single => draw_rect(intersection_rect, color),
                Crossing::TopLeftToBottomRigt => {
                    draw_rect(intersection_rect, color);
                    draw_line_v(top_left, bottom_right, TRIANGLE_BORDER);
                }
                Crossing::TopRightToBottomLeft => {
                    draw_rect(intersection_rect, color);
                    draw_line_v(top_right, bottom_left, TRIANGLE_BORDER);
                }
                Crossing::VerticalOnTop => {
                    draw_rect(intersection_rect, color);
                    draw_line_v(top_right, bottom_right, TRIANGLE_BORDER);
                    draw_line_v(top_left, bottom_left, TRIANGLE_BORDER);
                }
                Crossing::HorizontalOnTop => {
                    draw_rect(intersection_rect, color);
                    draw_line_v(top_right, top_left, TRIANGLE_BORDER);
                    draw_line_v(bottom_right, bottom_left, TRIANGLE_BORDER);
                }
            }
        }
    }
}

pub fn render_constraints(constraints: &Constraints, grid: &Grid, theme: &Theme) {
    let triangle_half_width = 4.0 * CELL_PAD;
    let small_triangle_half_width = 2.0 * CELL_PAD;
    let thickness = 1.5 * CELL_PAD;
    enum Constraint {
        Station,
        Blockade,
    }
    for constraint in &constraints.rails {
        let (color, color_border) = if matches_constraint_and_reachable(grid, constraint) {
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

        let corner = top_left_rail_intersection(row, column, theme);
        let reverse = direction.x + direction.y < 0.0;
        let start =
            corner - reverse as i32 as f32 * direction * (theme.layout.cell_width() + CELL_PAD);
        let end = start + direction * (theme.layout.cell_width() + CELL_PAD);
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
    for row in 1..grid.fixed_rails.horiz_rows() {
        for column in 1..grid.fixed_rails.horiz_columns() {
            let user_constraint = grid.fixed_rails.get_horiz(row, column);
            if user_constraint {
                let direction = vec2(1.0, 0.0);
                render_user_rail_constraint(
                    small_triangle_half_width,
                    thickness,
                    row,
                    column,
                    direction,
                    theme,
                );
            }
        }
    }
    for row in 1..grid.fixed_rails.vert_rows() {
        for column in 1..grid.fixed_rails.vert_columns() {
            let user_constraint = grid.fixed_rails.get_vert(row, column);
            if user_constraint {
                let direction = vec2(0.0, 1.0);
                render_user_rail_constraint(
                    small_triangle_half_width,
                    thickness,
                    row,
                    column,
                    direction,
                    theme,
                );
            }
        }
    }
}

fn render_user_rail_constraint(
    small_triangle_half_width: f32,
    thickness: f32,
    row: i32,
    column: i32,
    direction: Vec2,
    theme: &Theme,
) {
    let start = top_left_rail_intersection(row, column, theme);
    let end = start + direction * (theme.layout.cell_width() + CELL_PAD);
    let mid = (start + end) * 0.5;
    let diff = (end - start).normalize();
    let to_left = vec2(diff.y, -diff.x);

    let forward = diff * thickness * 0.5;
    let leftward = to_left * small_triangle_half_width;
    let a = mid + forward + leftward;
    let b = mid + forward - leftward;
    let c = mid - forward + leftward;
    let d = mid - forward - leftward;
    draw_triangle(a, c, b, RAIL);
    draw_triangle(b, c, d, RAIL);
    draw_lines(&[a, b, d, c, a], TRIANGLE_BORDER);
}

fn top_left_rail_intersection(i_row: i32, i_column: i32, theme: &Theme) -> Vec2 {
    cell_top_left(i_row, i_column, theme) - CELL_PAD * 0.5
}

fn cell_top_left(i_row: i32, i_column: i32, theme: &Theme) -> Vec2 {
    let x = GRID_PAD + i_column as f32 * (theme.layout.cell_width() + CELL_PAD) + 0.5;
    let y = GRID_PAD + i_row as f32 * (theme.layout.cell_height() + CELL_PAD) + 0.5;
    vec2(x, y)
}

pub fn draw_bordered_triangle(p_1: Vec2, p_2: Vec2, p_3: Vec2, color: Color, border: Color) {
    draw_triangle(p_1, p_2, p_3, color);
    draw_triangle_lines(p_1, p_2, p_3, 1.0, border);
}

fn render_tick(anchor: Anchor, size: f32) {
    let rect = anchor.get_rect(vec2(size, size));
    draw_rect(rect, SUCCESS_DARK);
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
