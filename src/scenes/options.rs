use crate::theme::{new_button, new_button_group_direction, render_button, render_text, Theme};
use crate::{new_layout, AnyError, NextStage, BACKGROUND, PANEL_BACKGROUND, TEXT_STYLE};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button_group;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::math::Rect;
use macroquad::prelude::{clear_background, next_frame, screen_height, screen_width, vec2, Vec2};
use std::ops::{Add, Sub};

pub async fn options(theme: &mut Theme) -> Result<NextStage, AnyError> {
    let mut screen = vec2(screen_width(), screen_height());
    loop {
        let new_screen = vec2(screen_width(), screen_height());
        if new_screen != screen {
            screen = new_screen;
            theme.layout = new_layout(screen.x, screen.y);
        }
        let panel = Rect::new(
            theme.grid_pad(),
            theme.grid_pad(),
            screen.x - 2.0 * theme.grid_pad(),
            screen.y - 2.0 * theme.grid_pad(),
        );

        clear_background(BACKGROUND);
        draw_rect(panel, PANEL_BACKGROUND);

        let mut point = vec2(panel.center().x, panel.y + theme.button_pad());
        for f in [change_font_ui, change_rows, change_columns, change_volume] {
            point = f(theme, point);
            point += vec2(0.0, theme.button_pad());
        }

        let anchor_point = vec2(panel.center().x, panel.bottom() - theme.button_pad());
        let mut back = new_button("Back", Anchor::bottom_center_v(anchor_point), theme);
        if is_key_pressed(KeyCode::Escape) || back.interact().is_clicked() {
            return Ok(NextStage::MainMenu);
        }
        render_button(&back);
        next_frame().await
    }
}

fn change_font_ui(theme: &mut Theme, anchor_point: Vec2) -> Vec2 {
    let font_size = theme.font_size();
    let text = format!("font size: {}", font_size);
    let (value, new_anchor_point) = inc_dec(theme, anchor_point, text, font_size, 1.0);
    *theme.font_size_mut() = value;
    new_anchor_point
}
fn change_rows(theme: &mut Theme, anchor_point: Vec2) -> Vec2 {
    let rows = theme.preferred_rows();
    let text = format!("Rows in procedural levels: {}", rows);
    let (value, new_anchor_point) = inc_dec(theme, anchor_point, text, rows, 1);
    let value = value.clamp(4, 50);
    *theme.preferred_rows_mut() = value;
    new_anchor_point
}
fn change_columns(theme: &mut Theme, anchor_point: Vec2) -> Vec2 {
    let columns = theme.preferred_columns();
    let text = format!("Columns in procedural levels: {}", columns);
    let (value, new_anchor_point) = inc_dec(theme, anchor_point, text, columns, 1);
    let value = value.clamp(3, 51);
    *theme.preferred_columns_mut() = value;
    new_anchor_point
}
fn change_volume(theme: &mut Theme, anchor_point: Vec2) -> Vec2 {
    let columns = theme.volume();
    let text = format!("Audio volume: {:.2}", columns);
    let (value, new_anchor_point) = inc_dec(theme, anchor_point, text, columns, 0.05);
    let value = value.clamp(0.0, 1.0);
    theme.set_volume(value);
    new_anchor_point
}
fn inc_dec<T: Add<Output = T> + Sub<Output = T>>(
    theme: &Theme,
    anchor_point: Vec2,
    text: String,
    mut value: T,
    change: T,
) -> (T, Vec2) {
    let half_pad = vec2(theme.cell_pad() * 0.5, 0.0);

    let anchor = Anchor::top_right_v(anchor_point - half_pad);
    let title = theme.new_text(&text, anchor);

    let anchor = Anchor::top_left_v(anchor_point + half_pad);
    let labels = new_button_group_direction(anchor, theme, button_group::Direction::Right);
    let [mut increase, mut decrease] = labels.create(["increase", "decrease"]);

    if increase.interact().is_clicked() {
        value = value + change;
    } else if decrease.interact().is_clicked() {
        value = value - change;
    }
    render_text(&title, &TEXT_STYLE);
    render_button(&increase);
    render_button(&decrease);
    (value, anchor_point + vec2(0.0, increase.rect().h))
}
