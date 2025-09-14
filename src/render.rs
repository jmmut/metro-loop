
use crate::logic::constraints::{matches_constraint_and_reachable, Reverse, Satisfaction};
use crate::logic::grid::get_cell;
use crate::logic::intersection::{Crossing, Intersection};
use crate::theme::{new_button, new_text, new_text_group, render_text, render_tooltip, Theme};
use crate::*;
use juquad::draw::draw_rect;
use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::button::Button;
use juquad::widgets::Widget;
use macroquad::math::f32;
use macroquad::prelude::*;

#[derive(PartialEq)]
pub enum RenderRail {
    Some {
        reachable: bool,
        start: IVec2,
        end: IVec2,
        coord: IVec2,
    },
    None,
}

pub fn is_horizontal_center(horizontal: Horizontal) -> bool {
    horizontal.opposite() == horizontal
}

pub fn render_cells(grid: &Grid, hovered_cell: &Option<(i32, i32)>, theme: &Theme) {
    for i_row in 0..grid.rows() {
        for i_column in 0..grid.columns() {
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
                theme.cell_width(),
                theme.cell_height(),
                color,
            );
        }
    }
}
pub fn render_grid(grid: &Grid, theme: &Theme) {
    for i_row in 0..grid.rows() {
        for i_column in 0..grid.columns() {
            let current_cell = *get(&grid.fixed_cells, i_row, i_column);
            if current_cell {
                let mut intersection = top_left_rail_intersection(i_row, i_column, theme);
                intersection += vec2(theme.cell_width(), theme.cell_height()) * 0.5;
                draw_rect(
                    Rect::new(
                        intersection.x,
                        intersection.y,
                        theme.cell_pad(),
                        theme.cell_pad(),
                    ),
                    TRIANGLE_BORDER,
                );
            }
        }
    }
    // horizontal rails
    for i_row in 1..grid.rails.horiz_rows() - 1 {
        for i_column in 1..grid.rails.horiz_columns() - 1 {
            let direction = grid.rails.get_horiz(i_row, i_column);
            let is_center = direction == direction.opposite();
            let rail = if is_center {
                RenderRail::None
            } else {
                let (start, end) = match direction {
                    Horizontal::Left => (ivec2(1, 0), ivec2(0, 0)),
                    Horizontal::Center | Horizontal::Right => (ivec2(0, 0), ivec2(1, 0)),
                };
                let reachable = grid.reachable_rails.get_horiz(i_row, i_column);
                RenderRail::Some {
                    reachable,
                    start,
                    end,
                    coord: ivec2(i_column, i_row),
                }
            };
            render_rail(rail, theme);
        }
    }

    // vertical rails
    for i_row in 1..grid.rails.vert_rows() - 1 {
        for i_column in 1..grid.rails.vert_columns() - 1 {
            let direction = grid.rails.get_vert(i_row, i_column);
            let is_center = direction == direction.opposite();
            let rail = if is_center {
                RenderRail::None
            } else {
                let (start, end) = match direction {
                    Vertical::Top => (ivec2(0, 1), ivec2(0, 0)),
                    Vertical::Center | Vertical::Bottom => (ivec2(0, 0), ivec2(0, 1)),
                };
                let reachable = grid.reachable_rails.get_vert(i_row, i_column);
                RenderRail::Some {
                    reachable,
                    start,
                    end,
                    coord: ivec2(i_column, i_row),
                }
            };
            render_rail(rail, theme);
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
            let top_left = bottom_right - theme.cell_pad();
            let top_right = top_left + vec2(theme.cell_pad(), 0.0);
            let bottom_left = top_left + vec2(0.0, theme.cell_pad());
            let intersection_rect =
                Rect::new(top_left.x, top_left.y, theme.cell_pad(), theme.cell_pad());
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
                    let top_left = top_left + vec2(-0.5, 0.0);
                    let bottom_left = bottom_left + vec2(-0.5, 0.0);
                    let top_right = top_right + vec2(0.5, 0.0);
                    let bottom_right = bottom_right + vec2(0.5, 0.0);
                    draw_line_v(top_right, bottom_right, TRIANGLE_BORDER);
                    draw_line_v(top_left, bottom_left, TRIANGLE_BORDER);
                }
                Crossing::HorizontalOnTop => {
                    draw_rect(intersection_rect, color);
                    let top_left = top_left + vec2(0.0, -0.5);
                    let bottom_left = bottom_left + vec2(0.0, 0.5);
                    let top_right = top_right + vec2(0.0, -0.5);
                    let bottom_right = bottom_right + vec2(0.0, 0.5);
                    draw_line_v(top_right, top_left, TRIANGLE_BORDER);
                    draw_line_v(bottom_right, bottom_left, TRIANGLE_BORDER);
                }
            }
        }
    }
}

pub fn render_rail(render_rail: RenderRail, theme: &Theme) {
    match render_rail {
        RenderRail::None => {}
        RenderRail::Some {
            reachable,
            start,
            end,
            coord,
        } => {
            let color = if reachable { RAIL } else { UNREACHABLE_RAIL };
            let start = coord + start;
            let end = coord + end;
            let start = top_left_rail_intersection(start.y, start.x, theme);
            let end = top_left_rail_intersection(end.y, end.x, theme);
            draw_line(start.x, start.y, end.x, end.y, theme.cell_pad(), color);
            let direction = (end - start).normalize();
            let border_start = start + direction * theme.cell_pad() * 0.5;
            let leftwards = vec2(direction.y, -direction.x);
            let left_border_start = border_start + leftwards * (theme.cell_pad() * 0.5 + 0.5);
            let right_border_start = border_start - leftwards * (theme.cell_pad() * 0.5 + 0.5);
            let border_end = end - direction * theme.cell_pad() * 0.5;
            let left_border_end = border_end + leftwards * (theme.cell_pad() * 0.5 + 0.5);
            let right_border_end = border_end - leftwards * (theme.cell_pad() * 0.5 + 0.5);
            draw_line_v(left_border_start, left_border_end, TRIANGLE_BORDER);
            draw_line_v(right_border_start, right_border_end, TRIANGLE_BORDER);

            if reachable {
                calculate_and_draw_triangle(
                    theme,
                    color,
                    start,
                    end,
                    direction,
                    leftwards,
                    TRIANGLE_BORDER,
                );
            }
        }
    }
}

fn calculate_and_draw_triangle(
    theme: &Theme,
    color: Color,
    start: Vec2,
    end: Vec2,
    direction: Vec2,
    leftwards: Vec2,
    color_border: Color,
) {
    let mid = (start + end) * 0.5;
    let triangle_width = 2.0 * theme.cell_pad();
    let left = mid + leftwards * triangle_width;
    let right = mid - leftwards * triangle_width;
    let tip = mid + triangle_width * direction;
    draw_bordered_triangle(left, right, tip, color, color_border);
}

pub fn render_constraints(constraints: &Constraints, grid: &Grid, theme: &Theme) {
    let triangle_half_width = 4.0 * theme.cell_pad();
    let small_triangle_half_width = 2.0 * theme.cell_pad();
    enum Constraint {
        Station,
        Blockade,
    }
    for constraint in &constraints.rails {
        let (success, reversed_rail) = matches_constraint_and_reachable(grid, constraint);
        let (color, color_border) = if success {
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
            corner - reverse as i32 as f32 * direction * (theme.cell_width() + theme.cell_pad());
        let end = start + direction * (theme.cell_width() + theme.cell_pad());
        let mid = (start + end) * 0.5;
        let mut diff = (end - start).normalize();
        let to_left = vec2(diff.y, -diff.x);

        match constraint_render {
            Constraint::Station => {
                let left = mid + to_left * small_triangle_half_width;
                let right = mid - to_left * small_triangle_half_width;
                let outer_left = mid + to_left * triangle_half_width;
                let outer_right = mid - to_left * triangle_half_width;
                let tip = mid + diff * small_triangle_half_width;
                let outer_tip = mid + diff * triangle_half_width;

                // let daltonic_distinction = if success { color } else { color_border };

                draw_triangle(outer_left, left, outer_tip, color);
                draw_triangle(left, tip, outer_tip, color);
                draw_triangle(right, outer_tip, tip, color);
                draw_triangle(right, outer_right, outer_tip, color);

                draw_lines(
                    &[tip, left, outer_left, outer_tip, outer_right, right, tip],
                    color_border,
                );
                if !success {
                    if reversed_rail != Reverse::None {
                        diff *= -1.0;
                        calculate_and_draw_triangle(
                            theme,
                            color_border,
                            start,
                            end,
                            diff,
                            to_left,
                            color,
                        );
                    }
                }
            }
            Constraint::Blockade => {
                draw_blockade(
                    theme,
                    small_triangle_half_width,
                    success,
                    color,
                    color_border,
                    mid,
                    diff,
                    to_left,
                    start,
                    end,
                    reversed_rail.is_reverse(),
                );
            }
        }
    }
    for row in 1..grid.fixed_rails.horiz_rows() {
        for column in 1..grid.fixed_rails.horiz_columns() {
            let user_constraint = grid.fixed_rails.get_horiz(row, column);
            if user_constraint {
                let direction = vec2(1.0, 0.0);
                let constraint = RailCoord::Horizontal {
                    row,
                    column,
                    direction: Horizontal::Center,
                };
                render_user_rail_constraint(
                    small_triangle_half_width,
                    row,
                    column,
                    direction,
                    constraint,
                    grid,
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
                let constraint = RailCoord::Vertical {
                    row,
                    column,
                    direction: Vertical::Center,
                };
                render_user_rail_constraint(
                    small_triangle_half_width,
                    row,
                    column,
                    direction,
                    constraint,
                    grid,
                    theme,
                );
            }
        }
    }
}

fn draw_blockade(
    theme: &Theme,
    small_triangle_half_width: f32,
    success: bool,
    color: Color,
    color_border: Color,
    mid: Vec2,
    mut direction: Vec2,
    to_left: Vec2,
    start: Vec2,
    end: Vec2,
    reverse: bool,
) {
    let forward = direction * small_triangle_half_width * 1.25;
    let leftward = to_left * small_triangle_half_width * 1.25;
    let a = mid + forward + leftward;
    let b = mid + forward - leftward;
    let c = mid - forward + leftward;
    let d = mid - forward - leftward;
    // let (blockade_color, blockade_color_border) = if success {
    //     (color, color_border)
    // } else {
    //     (color_border, color)
    // };
    // if success {
    draw_triangle(a, c, b, color);
    draw_triangle(b, c, d, color);
    // draw_triangle(a, c, b, blockade_color);
    // draw_triangle(b, c, d, blockade_color);
    // }
    // draw_lines(&[a, b, d, c, a], color_border);
    // draw_lines(&[d, c], blockade_color_border);
    // draw_lines(&[a, b], blockade_color_border);
    draw_lines(&[d, c], color_border);
    draw_lines(&[a, b], color_border);

    if !success {
        if reverse {
            direction *= -1.0;
        }
        // calculate_and_draw_triangle(theme, color, start, end, direction, to_left, color_border);
        calculate_and_draw_triangle(
            theme,
            color_border,
            start,
            end,
            direction,
            to_left,
            color_border,
        );
        // calculate_and_draw_triangle(theme, RAIL, start, end, direction, to_left, TRIANGLE_BORDER);

        //     let forward = diff * thickness * 0.5;
        //     let leftward = to_left * theme.cell_pad() * 0.75;
        //     let a = mid + forward + leftward;
        //     let b = mid + forward - leftward;
        //     let c = mid - forward + leftward;
        //     let d = mid - forward - leftward;
        //     draw_triangle(a, c, b, color_border);
        //     draw_triangle(b, c, d, color_border);
    }
}

fn render_user_rail_constraint(
    small_triangle_half_width: f32,
    row: i32,
    column: i32,
    direction: Vec2,
    constraint: RailCoord,
    grid: &Grid,
    theme: &Theme,
) {
    let start = top_left_rail_intersection(row, column, theme);
    let end = start + direction * (theme.cell_width() + theme.cell_pad());
    let mid = (start + end) * 0.5;
    let diff = (end - start).normalize();
    let to_left = vec2(diff.y, -diff.x);
    let (success, reverse) = matches_constraint_and_reachable(grid, &constraint);
    draw_blockade(
        theme,
        small_triangle_half_width,
        success,
        RAIL,
        TRIANGLE_BORDER,
        mid,
        diff,
        to_left,
        start,
        end,
        reverse.is_reverse(),
    );
}

fn top_left_rail_intersection(i_row: i32, i_column: i32, theme: &Theme) -> Vec2 {
    cell_top_left(i_row, i_column, theme) - theme.cell_pad() * 0.5
}

fn cell_top_left(i_row: i32, i_column: i32, theme: &Theme) -> Vec2 {
    let x = theme.grid_pad() + i_column as f32 * (theme.cell_width() + theme.cell_pad());
    let y = theme.grid_pad() + i_row as f32 * (theme.cell_height() + theme.cell_pad());
    vec2(x, y)
}

pub fn draw_bordered_triangle(p_1: Vec2, p_2: Vec2, p_3: Vec2, color: Color, border: Color) {
    draw_triangle(p_1, p_2, p_3, color);
    draw_triangle_lines(p_1, p_2, p_3, 1.0, border);
}

pub fn render_tick(anchor: Anchor, size: f32, theme: &Theme) {
    let rect = anchor.get_rect(vec2(size, size));
    draw_rect(rect, SUCCESS_DARK);
    let start = rect.point() + rect.size() * 0.25;
    let mid = rect.point() + rect.size() * 0.5;
    let end = rect.point() + rect.size() * 0.75;
    draw_line(start.x, mid.y, mid.x, end.y, theme.cell_pad(), SUCCESS);
    draw_line(
        mid.x - theme.cell_pad() * 0.5,
        end.y,
        end.x,
        start.y,
        theme.cell_pad(),
        SUCCESS,
    );
}

pub fn render_cross(anchor: Anchor, font_size: f32, theme: &Theme) {
    let rect = anchor.get_rect(vec2(font_size, font_size));
    draw_rect(rect, FAILING_DARK);
    let start = rect.point() + rect.size() * 0.25;
    let end = rect.point() + rect.size() * 0.75;
    draw_line(start.x, start.y, end.x, end.y, theme.cell_pad(), FAILING);
    draw_line(start.x, end.y, end.x, start.y, theme.cell_pad(), FAILING);
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
