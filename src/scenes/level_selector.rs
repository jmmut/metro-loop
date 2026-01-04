use crate::level_history::{GameTrack, Solved};
use crate::render::cell_top_left;
use crate::scenes::play::default_pixel_to_coord;
use crate::theme::{new_button, new_imm_button, new_text, render_button, render_text, Theme};
use crate::{
    new_layout, AnyError, NextStage, BACKGROUND, DISABLED_CELL, ENABLED_CELL, HOVERED_CELL,
    PANEL_BACKGROUND, TEXT_STYLE,
};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::Widget;
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::math::Vec2;
use macroquad::prelude::{
    clear_background, draw_rectangle, next_frame, screen_height, screen_width, vec2,
};

pub async fn level_selector(
    theme: &mut Theme,
    game_track: &mut GameTrack,
) -> Result<NextStage, AnyError> {
    let mut screen = vec2(screen_width(), screen_height());
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
    theme.layout.resize_grid_mut(longest_section, grid_sections);
    let mut selected_level = None;
    loop {
        let new_screen = vec2(screen_width(), screen_height());
        if new_screen != screen {
            screen = new_screen;
            theme.layout = new_layout(screen.x, screen.y)
                .resize_grid(longest_section as i32, grid_sections as i32);
        }

        let panel = theme.default_button_panel_rect();

        let pos = Vec2::from(mouse_position());
        let hovered_cell = default_pixel_to_coord(pos, &theme);

        clear_background(BACKGROUND);
        render_solved(&game_track.solved, &hovered_cell, theme);
        draw_rect(panel, PANEL_BACKGROUND);

        if is_mouse_button_pressed(MouseButton::Left) {
            if !panel.contains(pos) {
                selected_level = hovered_cell;
            };
        }

        if let Some((i_row, i_column)) = selected_level.clone() {
            if game_track.select(i_row, i_column, &theme.resources.levels) {
                let anchor =
                    Anchor::from_top(panel, Horizontal::Center, Vec2::splat(theme.grid_pad()));
                let level_name = game_track.current.to_string();
                let title = new_text(&level_name, anchor, 1.0, theme);
                render_text(&title, &TEXT_STYLE);

                let anchor = Anchor::below(title.rect(), Horizontal::Center, theme.button_margin());
                let (_rect, interaction) = new_imm_button("PLAY", anchor, theme);
                if interaction.is_clicked() {
                    return Ok(NextStage::Campaign);
                }
            }
        }
        let anchor_bottom = Anchor::from_bottom(
            panel,
            Horizontal::Center,
            Vec2::splat(theme.button_margin()),
        );
        let mut main_menu = new_button("MENU", anchor_bottom, theme);
        if main_menu.interact().is_clicked() {
            return Ok(NextStage::MainMenu);
        }
        render_button(&main_menu);
        next_frame().await;
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
