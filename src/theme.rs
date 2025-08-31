use crate::scenes::loading_screen::Resources;
use crate::STYLE;
use juquad::draw::draw_rect;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::Button;
use juquad::widgets::text::TextRect;
use juquad::widgets::StateStyle;
use juquad::widgets::Widget;
use macroquad::prelude::Font;

pub struct Theme {
    pub resources: Resources,
    pub layout: Layout,
}

pub struct Layout {
    font_size: f32,
    cell_width: f32,
    cell_height: f32,
}

impl Layout {
    pub const fn new(font_size: f32, cell_width: f32, cell_height: f32) -> Self {
        Self { font_size, cell_width, cell_height }
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
