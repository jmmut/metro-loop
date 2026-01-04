use crate::level_history::{GameTrack, Solved};
use crate::render::{cell_top_left, render_rail, RenderRail};
use crate::scenes::play::default_pixel_to_coord;
use crate::scenes::play::panel::{get_icon_rect, render_tick_or_cross};
use crate::theme::{new_imm_button, new_text, render_text, render_tooltip, Theme};
use crate::{
    new_layout, AnyError, NextStage, BACKGROUND, DISABLED_CELL, ENABLED_CELL, HOVERED_CELL,
    PANEL_BACKGROUND, RAIL, TEXT_STYLE, UNREACHABLE_RAIL,
};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::Widget;
use macroquad::input::{
    is_key_pressed, is_mouse_button_pressed, mouse_position, KeyCode, MouseButton,
};
use macroquad::math::{IVec2, Rect, Vec2};
use macroquad::prelude::{
    clear_background, draw_rectangle, next_frame, screen_height, screen_width, vec2,
};

pub async fn level_selector(
    theme: &mut Theme,
    game_track: &mut GameTrack,
) -> Result<NextStage, AnyError> {
    let mut screen = vec2(screen_width(), screen_height());
    let button_margin_v = Vec2::splat(theme.button_margin());
    let longest_section = theme
        .resources
        .levels
        .sections
        .iter()
        .max_by(|a, b| a.levels.len().cmp(&b.levels.len()))
        .unwrap()
        .levels
        .len() as i32;
    let grid_sections = 8.max(theme.resources.levels.sections.len()) as i32;
    theme.layout.resize_grid_mut(grid_sections, longest_section);
    let mut selected_level = game_track.get_next_unsolved_ids(theme).await;
    loop {
        let new_screen = vec2(screen_width(), screen_height());
        if new_screen != screen {
            screen = new_screen;
            theme.layout =
                new_layout(screen.x, screen.y).resize_grid(grid_sections, longest_section);
        }

        let mouse_pos = Vec2::from(mouse_position());
        let hovered_cell = default_pixel_to_coord(mouse_pos, &theme);

        clear_background(BACKGROUND);
        render_solved(&game_track.solved, &hovered_cell, theme);

        let panel = theme.default_button_panel_rect();
        draw_rect(panel, PANEL_BACKGROUND);

        if is_mouse_button_pressed(MouseButton::Left) {
            if !panel.contains(mouse_pos) {
                selected_level = hovered_cell;
            };
        }

        if let Some((i_row, i_column)) = selected_level.clone() {
            if game_track.select(i_row, i_column, &theme.resources.levels) {
                let anchor = Anchor::from_top(panel, Horizontal::Center, button_margin_v);
                let level_name = game_track.current.to_string();
                let title = new_text(&level_name, anchor, 1.0, theme);
                render_text(&title, &TEXT_STYLE);

                let icon_rect = get_icon_rect(&title);
                let solved = game_track.is_solved(i_row, i_column);
                render_tick_or_cross(icon_rect, solved, theme);
                if icon_rect.contains(mouse_pos) || title.rect().contains(mouse_pos) {
                    let anchor = Anchor::bottom_right_v(mouse_pos);
                    let text = if solved {
                        "Level is solved"
                    } else {
                        "Level is unsolved"
                    };
                    let tooltip = new_text(text, anchor, 1.0, theme);
                    render_tooltip(&tooltip, &TEXT_STYLE)
                }

                let anchor = Anchor::below(title.rect(), Horizontal::Center, theme.button_margin());
                if new_imm_button("PLAY", anchor, theme).1.is_clicked() {
                    return Ok(NextStage::Campaign);
                }
                render_rails_on_selected_level(i_row, i_column, solved, theme);
            }
        }
        let anchor = Anchor::from_bottom(panel, Horizontal::Center, button_margin_v);
        if is_key_pressed(KeyCode::Escape) || new_imm_button("MENU", anchor, theme).1.is_clicked() {
            return Ok(NextStage::MainMenu);
        }

        next_frame().await;
    }
}

fn render_rails_on_selected_level(i_row: i32, i_column: i32, is_solved: bool, theme: &Theme) {
    let points = [(0, 0), (1, 0), (1, 1), (0, 1), (0, 0)];
    for i in 1..points.len() {
        let level_solved = is_solved;
        render_rail(
            RenderRail::Some {
                reachable: level_solved,
                start: IVec2::from(points[i - 1]),
                end: IVec2::from(points[i]),
                coord: IVec2::new(i_column, i_row),
            },
            theme,
        );
        let color = if level_solved { RAIL } else { UNREACHABLE_RAIL };

        let bottom_right = cell_top_left(i_row + points[i].1, i_column + points[i].0, theme);
        let top_left = bottom_right - theme.cell_pad();
        let intersection_rect =
            Rect::new(top_left.x, top_left.y, theme.cell_pad(), theme.cell_pad());
        draw_rect(intersection_rect, color);
    }
}

pub fn render_solved(solved: &Solved, hovered_cell: &Option<(i32, i32)>, theme: &Theme) {
    for (i_row, section) in solved.iter().enumerate() {
        for (i_column, level) in section.iter().enumerate() {
            let i_row = i_row as i32;
            let i_column = i_column as i32;
            let color = if *hovered_cell == Some((i_row, i_column)) {
                HOVERED_CELL
            } else {
                if *level {
                    ENABLED_CELL
                } else {
                    DISABLED_CELL
                }
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
