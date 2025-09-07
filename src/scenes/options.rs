use crate::theme::{new_button, new_button_group_direction, render_button, render_text, Theme};
use crate::{new_layout, AnyError, NextStage, BACKGROUND, PANEL_BACKGROUND, STYLE};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button_group;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::math::Rect;
use macroquad::prelude::{clear_background, next_frame, screen_height, screen_width, vec2};

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

        change_font_ui(panel, theme);
        let anchor_point = vec2(panel.center().x, panel.bottom() - theme.button_pad());
        let mut back = new_button("Back", Anchor::bottom_center_v(anchor_point), theme);
        if is_key_pressed(KeyCode::Escape) || back.interact().is_clicked() {
            return Ok(NextStage::MainMenu);
        }
        render_button(&back);
        next_frame().await
    }
}

fn change_font_ui(button_panel: Rect, theme: &mut Theme) {
    let half_pad = vec2(theme.cell_pad() * 0.5, 0.0);
    let anchor_point = button_panel.center();

    let anchor = Anchor::bottom_right_v(anchor_point - half_pad);
    let title = theme.new_text(&format!("font size: {}", theme.font_size()), anchor);

    let anchor = Anchor::bottom_left_v(anchor_point + half_pad);
    let labels = new_button_group_direction(anchor, theme, button_group::Direction::Right);
    let [mut increase, mut decrease] = labels.create(["increase", "decrease"]);

    if increase.interact().is_clicked() {
        *theme.font_size_mut() += 1.0;
    }
    if decrease.interact().is_clicked() {
        *theme.font_size_mut() -= 1.0;
    }
    render_text(&title, &STYLE.at_rest);
    render_button(&increase);
    render_button(&decrease);
}
