use crate::scenes::loading_screen::Resources;
use crate::{choose_scale, CELL_PAD, GRID_PAD, NUM_COLUMNS, NUM_ROWS, STYLE};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::Button;
use juquad::widgets::text::TextRect;
use juquad::widgets::StateStyle;
use juquad::widgets::Widget;
use macroquad::math::{f32, Rect};
use macroquad::prelude::Font;

pub struct Theme {
    pub resources: Resources,
    pub layout: Layout,
}

pub struct Layout {
    screen_width: f32,
    screen_height: f32,
    font_size: f32,
    cell_width: f32,
    cell_height: f32,
}

impl Layout {
    pub fn new(
        screen_width: f32,
        screen_height: f32,
        mut font_size: f32,
        _cell_width: f32,
        _cell_height: f32,
    ) -> Self {
        let update_scale = |value: &mut f32| {
            *value = choose_scale(screen_width, screen_height, *value);
        };
        update_scale(&mut font_size);

        let screen_height_proportional = screen_width * 9.0 / 16.0;
        let screen_height = screen_height_proportional.min(screen_height);
        let screen_width = screen_height * 16.0 / 9.0;

        let cell_height = (screen_height - GRID_PAD * 2.0 + CELL_PAD) / NUM_ROWS as f32 - CELL_PAD;
        let cell_width = cell_height;
        Self {
            screen_width,
            screen_height,
            font_size,
            cell_width,
            cell_height,
        }
    }

    pub fn useable_screen_size(&self) -> (f32, f32) {
        (self.screen_width, self.screen_height)
    }
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
    pub fn font_size_mut(&mut self) -> &mut f32 {
        &mut self.font_size
    }
    pub fn cell_width(&self) -> f32 {
        self.cell_width
    }
    pub fn cell_height(&self) -> f32 {
        self.cell_height
    }
}
pub fn grid_width(theme: &Theme) -> f32 {
    (theme.layout.cell_width() + CELL_PAD) * NUM_COLUMNS as f32 - CELL_PAD
}

pub fn grid_height(theme: &Theme) -> f32 {
    (theme.layout.cell_height() + CELL_PAD) * NUM_ROWS as f32 - CELL_PAD
}

pub fn button_panel_width(theme: &Theme) -> f32 {
    let (sw, _sh) = theme.layout.useable_screen_size();
    sw - GRID_PAD * 3.0 - grid_width(theme)
}

pub fn button_panel_rect(theme: &mut Theme) -> Rect {
    Rect::new(
        grid_width(theme) + GRID_PAD * 2.0,
        GRID_PAD,
        button_panel_width(theme),
        grid_height(theme),
    )
}

pub fn new_button(text: &str, anchor: Anchor, theme: &Theme) -> Button {
    let text_rect = new_text(text, anchor, 1.0, theme);
    Button::new_from_text_rect(text_rect)
}
pub fn render_button(button: &Button) {
    button.render_default(&STYLE);
}

pub fn new_text(text: &str, anchor: Anchor, size_coef: f32, theme: &Theme) -> TextRect {
    new_text_internal(
        text,
        anchor,
        size_coef,
        &theme.layout,
        Some(theme.resources.font),
    )
}
pub fn new_text_unloaded(text: &str, anchor: Anchor, size_coef: f32, layout: &Layout) -> TextRect {
    new_text_internal(text, anchor, size_coef, layout, None)
}
fn new_text_internal(
    text: &str,
    anchor: Anchor,
    size_coef: f32,
    layout: &Layout,
    font: Option<Font>,
) -> TextRect {
    TextRect::new_generic(
        text,
        anchor,
        layout.font_size() * size_coef,
        font,
        macroquad::prelude::measure_text,
    )
}
pub fn render_text(text_rect: &TextRect, style: &StateStyle) {
    draw_rect(text_rect.rect(), style.bg_color);
    text_rect.render_default(style)
}
