use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::input::input_macroquad::InputMacroquad;
use juquad::input::input_trait::InputTrait;
use juquad::widgets::{Interaction, Style, Widget};
use macroquad::input::MouseButton;
use macroquad::prelude::Rect;

pub struct Slider {
    pub min: f32,
    pub max: f32,
    pub current: f32,
    pub rect: Rect,
    pub interaction: Interaction,
    pub render_pos: Option<f32>,
}

impl Slider {
    pub fn new(min: f32, max: f32, current: f32, rect: Rect) -> Self {
        Self {
            min,
            max,
            current,
            rect,
            interaction: Interaction::None,
            render_pos: None,
        }
    }
    pub fn interact(&mut self) -> f32 {
        let input = InputMacroquad;
        let mouse_pos = input.mouse_position();
        let range = self.max - self.min;
        let current_coef = (self.current - self.min) / range;
        let handle_width = self.handle_width();
        let mouse_coef = (mouse_pos - self.rect.point() - 0.5 * handle_width)
            / (self.rect.size() - handle_width);
        let (interaction, render_pos) = if self.rect.contains(mouse_pos) {
            if input.is_mouse_button_down(MouseButton::Left) {
                (Interaction::Pressing, mouse_coef.x)
            } else if input.is_mouse_button_released(MouseButton::Left) {
                (Interaction::Clicked, mouse_coef.x)
            } else {
                (Interaction::Hovered, current_coef)
            }
        } else {
            if self.interaction.is_down() && input.is_mouse_button_down(MouseButton::Left) {
                (Interaction::Pressing, mouse_coef.x)
            } else {
                (Interaction::None, current_coef)
            }
        };
        self.interaction = interaction;
        self.render_pos = Some(render_pos.clamp(self.min, self.max));
        self.current = range * self.render_pos.unwrap() + self.min;
        self.current
    }
    pub fn render(&self, style: &Style) {
        let state_style = style.choose(self.interaction);
        let center = self.rect.center();
        let guide_height = self.rect.h * 0.3;
        let guide_rect = Rect::new(
            self.rect.x,
            center.y - guide_height * 0.5,
            self.rect.w,
            guide_height,
        );
        draw_rect(guide_rect, style.at_rest.bg_color);
        draw_rect_lines(guide_rect, 2.0, style.at_rest.border_color);

        let width = self.handle_width();
        let handle_rect = Rect::new(
            (self.rect.w - width) * self.render_pos.unwrap() + self.rect.x,
            self.rect.y,
            width,
            self.rect.h,
        );
        draw_rect(handle_rect, state_style.bg_color);
        draw_rect_lines(handle_rect, 2.0, state_style.border_color);

        // draw_rect_lines(self.rect, 2.0, RED);
    }

    fn handle_width(&self) -> f32 {
        self.rect.h * 0.62
    }
}

impl Widget for Slider {
    fn rect(&self) -> Rect {
        self.rect
    }

    fn rect_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }
}
