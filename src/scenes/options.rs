use crate::slider::Slider;
use crate::theme::{new_button, new_button_group_direction, render_button, render_text, Theme};
use crate::{new_layout, AnyError, NextStage, BACKGROUND, PANEL_BACKGROUND, STYLE, TEXT_STYLE};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button_group;
use juquad::widgets::text::TextRect;
use juquad::widgets::Widget;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::math::Rect;
use macroquad::prelude::{clear_background, next_frame, screen_height, screen_width, vec2, Vec2};
use std::ops::{Add, Sub};

pub async fn options(theme: &mut Theme) -> Result<NextStage, AnyError> {
    let mut screen = vec2(screen_width(), screen_height());
    let (mut title, mut volume) = change_volume(theme, vec2(0.0, 0.0));
    loop {
        let new_screen = vec2(screen_width(), screen_height());
        if new_screen != screen {
            screen = new_screen;
            theme.layout = new_layout(screen.x, screen.y);
            (title, volume) = change_volume(theme, vec2(0.0, 0.0));
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
        for f in [change_font_ui, change_rows, change_columns] {
            point = f(theme, point);
            point += vec2(0.0, theme.button_pad());
        }
        point = interact_volume(&mut title, &mut volume, theme, point);
        point += vec2(0.0, theme.button_pad());

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
    let current = theme.font_size();
    let text = format!("font size: {}", current);
    let (new, new_anchor_point) = inc_dec(theme, anchor_point, text, current, 1.0);
    *theme.font_size_mut() = new;
    new_anchor_point
}
fn change_rows(theme: &mut Theme, anchor_point: Vec2) -> Vec2 {
    let current = theme.preferred_rows();
    let text = format!("Rows in procedural levels: {}", current);
    let (value, new_anchor_point) = inc_dec(theme, anchor_point, text, current, 1);
    let new = value.clamp(4, 50);
    *theme.preferred_rows_mut() = new;
    new_anchor_point
}
fn change_columns(theme: &mut Theme, anchor_point: Vec2) -> Vec2 {
    let current = theme.preferred_columns();
    let text = format!("Columns in procedural levels: {}", current);
    let (new, new_anchor_point) = inc_dec(theme, anchor_point, text, current, 1);
    let new = new.clamp(3, 51);
    *theme.preferred_columns_mut() = new;
    new_anchor_point
}
fn change_volume(theme: &mut Theme, anchor_point: Vec2) -> (TextRect, Slider) {
    let current = theme.volume();
    let text = format!("Audio volume: {:.2}", current);

    let half_pad = vec2(theme.cell_pad() * 0.5, 0.0);
    let anchor = Anchor::top_right_v(anchor_point - half_pad);
    let title = theme.new_text(&text, anchor);

    let anchor = Anchor::top_left_v(anchor_point + half_pad);
    let slider = Slider::new(0.0, 1.0, current, anchor.get_rect(title.rect().size()));
    (title, slider)
}

fn interact_volume(
    title: &mut TextRect,
    slider: &mut Slider,
    theme: &mut Theme,
    anchor_point: Vec2,
) -> Vec2 {
    let half_pad = vec2(theme.cell_pad() * 0.5, 0.0);
    let anchor = Anchor::top_left_v(anchor_point + half_pad);
    slider.reanchor(anchor);
    let new = slider.interact();
    slider.render(&STYLE);

    let new = new.clamp(0.0, 1.0);
    let current = theme.volume();
    theme.set_volume(new);
    fn stopped(value: f32) -> bool {
        value < 0.001
    }
    let text = if stopped(current) {
        "Audio volume: stopped".to_string()
    } else {
        format!("Audio volume: {:.2}", new)
    };
    if stopped(current) != stopped(new) {
        if stopped(new) {
            theme.resources.sounds.stop_background();
        } else {
            theme.resources.sounds.play_background();
        }
    }
    let anchor = Anchor::top_right_v(anchor_point - half_pad);
    *title = theme.new_text(&text, anchor);
    render_text(&title, &TEXT_STYLE);

    anchor_point + vec2(0.0, slider.rect().h)
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
    let [mut decrease, mut increase] = labels.create(["-", "+"]);

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
