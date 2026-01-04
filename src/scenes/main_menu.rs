use crate::theme::{labels_from_theme, new_button_group_generic, render_button, Theme};
use crate::{new_layout, AnyError, NextStage, BACKGROUND, PANEL_BACKGROUND};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::Button;
use juquad::widgets::button_group::LabelGroup;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::math::Rect;
use macroquad::prelude::{clear_background, next_frame, screen_height, screen_width, vec2};

pub struct Buttons {
    campaign: Button,
    options: Button,
    quit: Button,
}
impl Buttons {
    pub fn list(&self) -> Vec<&Button> {
        vec![&self.campaign, &self.options, &self.quit]
    }
}
impl From<[Button; 3]> for Buttons {
    fn from(value: [Button; 3]) -> Self {
        let [campaign, options, quit] = value;
        Self {
            campaign,
            options,
            quit,
        }
    }
}

pub async fn main_menu(theme: &mut Theme) -> Result<NextStage, AnyError> {
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
        let anchor = Anchor::center_v(panel.center());
        let labels = new_button_group_generic(
            theme,
            LabelGroup {
                font_size: 1.5,
                anchor,
                pad_x: Some(theme.button_margin() * 4.0),
                ..labels_from_theme(theme)
            },
        );
        let mut buttons: Buttons = labels.create(["CAMPAIGN", "OPTIONS", "QUIT"]).into();

        if buttons.campaign.interact().is_clicked() {
            return Ok(NextStage::LevelSelector);
        }
        if buttons.options.interact().is_clicked() {
            return Ok(NextStage::Options);
        }
        if is_key_pressed(KeyCode::Escape) || buttons.quit.interact().is_clicked() {
            return Ok(NextStage::Quit);
        }

        for b in buttons.list() {
            render_button(b);
        }
        next_frame().await;
    }
}
