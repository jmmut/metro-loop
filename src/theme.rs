use crate::scenes::loading_screen::Resources;
use crate::{choose_scale, CELL_PAD, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH, GRID_PAD, NUM_COLUMNS, STYLE};
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
    pub fn new(screen_width: f32, screen_height: f32, mut font_size: f32, mut cell_width: f32, mut cell_height: f32) -> Self {
        let update_scale = |value: &mut f32| {
            *value = choose_scale(screen_width, screen_height, *value);
        };
        update_scale(&mut font_size);
        let cells_per_screen_height = (DEFAULT_WINDOW_HEIGHT as f32 - GRID_PAD * 2.0 + CELL_PAD) / (cell_height+CELL_PAD);
        let cell_height_2 = (screen_height - GRID_PAD * 2.0 + CELL_PAD) / cells_per_screen_height - CELL_PAD;
        let cell_width_2 = cell_height_2;
        let panel_width = DEFAULT_WINDOW_WIDTH as f32 - GRID_PAD * 3.0 - cell_width_2 * NUM_COLUMNS as f32 - CELL_PAD * (NUM_COLUMNS -1) as f32;

        let grid_width = screen_width - GRID_PAD * 3.0 - screen_width * panel_width / DEFAULT_WINDOW_WIDTH as f32 ;
        let cell_height_as_per_width = (grid_width + CELL_PAD) / NUM_COLUMNS as f32;
        let cell_height_3 = cell_height_2.min(cell_height_as_per_width);
        let cell_width_3 = cell_height_3;
        Self { font_size, cell_width: cell_width_3, cell_height: cell_height_3 }
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
