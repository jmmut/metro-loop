use crate::logic::grid::Grid;
use crate::scenes::loading_screen::Resources;
use crate::{NUM_COLUMNS, NUM_ROWS, STYLE};
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::button::Button;
use juquad::widgets::button_group::{ButtonGroup, Direction, LabelGroup};
use juquad::widgets::text::TextRect;
use juquad::widgets::StateStyle;
use juquad::widgets::Widget;
use macroquad::math::{f32, Rect};
use macroquad::prelude::Font;

pub struct Theme {
    pub resources: Resources,
    pub layout: Layout,
    pub preferences: Preferences,
}

pub struct Preferences {
    pub rows: i32,
    pub columns: i32,
}

impl Preferences {
    pub fn new() -> Self {
        Self {
            rows: NUM_ROWS,
            columns: NUM_COLUMNS,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Layout {
    pub screen_width: f32,
    pub screen_height: f32,
    pub font_size: f32,
    pub cell_width: f32,
    pub cell_height: f32,
    pub grid_pad: f32,
    pub cell_pad: f32,
    pub default_rows: i32,
    pub default_columns: i32,
}

impl Layout {
    pub fn readjust(self) -> Self {
        let screen_height_proportional = self.screen_width * 9.0 / 16.0;
        let screen_height = screen_height_proportional.min(self.screen_height);
        let screen_width = screen_height * 16.0 / 9.0;

        let cell_height = (screen_height - self.grid_pad * 2.0 + self.cell_pad)
            / self.default_rows as f32
            - self.cell_pad;
        let cell_width = cell_height;
        Self {
            screen_width,
            screen_height,
            font_size: self.font_size,
            cell_width,
            cell_height,
            grid_pad: self.grid_pad,
            cell_pad: self.cell_pad,
            default_rows: self.default_rows,
            default_columns: self.default_columns,
        }
    }
    pub fn resize_grid(mut self, new_rows: i32, new_columns: i32) -> Self {
        self.resize_grid_mut(new_rows, new_columns);
        self
    }
    /// for some reason, `a = A {..a}` works, but if we put that inside a function
    /// `fn recreate(self) -> A { A {..self} }`, then `a = a.recreate()` doesn't work.
    /// That's why we need this second function that takes mut refs.
    pub fn resize_grid_mut(&mut self, new_rows: i32, new_columns: i32) {
        *self = Layout {
            default_rows: new_rows,
            default_columns: new_columns,
            ..*self
        }
        .readjust()
    }
    pub fn grid_pad(&self) -> f32 {
        self.grid_pad
    }
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
    pub fn cell_pad(&self) -> f32 {
        self.cell_pad
    }
}
impl Theme {
    pub fn useable_screen_size(&self) -> (f32, f32) {
        (self.layout.screen_width, self.layout.screen_height)
    }
    pub fn font_size(&self) -> f32 {
        self.layout.font_size
    }
    pub fn font_size_mut(&mut self) -> &mut f32 {
        &mut self.layout.font_size
    }
    pub fn cell_width(&self) -> f32 {
        self.layout.cell_width
    }
    pub fn cell_height(&self) -> f32 {
        self.layout.cell_height
    }
    pub fn grid_pad(&self) -> f32 {
        self.layout.grid_pad
    }
    pub fn cell_pad(&self) -> f32 {
        self.layout.cell_pad
    }
    pub fn default_rows(&self) -> i32 {
        self.layout.default_rows
    }
    pub fn default_columns(&self) -> i32 {
        self.layout.default_columns
    }
    pub fn grid_width(&self, grid: &Grid) -> f32 {
        (self.cell_width() + self.cell_pad()) * grid.columns() as f32 - self.cell_pad()
    }

    pub fn grid_height(&self, grid: &Grid) -> f32 {
        (self.cell_height() + self.cell_pad()) * grid.rows() as f32 - self.cell_pad()
    }

    pub fn button_panel_width(&self, grid: &Grid) -> f32 {
        let (sw, _sh) = self.useable_screen_size();
        sw - self.grid_pad() * 3.0 - self.grid_width(grid)
    }
    pub fn button_pad(&self) -> f32 {
        self.grid_pad()
    }
    pub fn button_panel_rect(&self, grid: &Grid) -> Rect {
        Rect::new(
            self.grid_width(grid) + self.grid_pad() * 2.0,
            self.grid_pad(),
            self.button_panel_width(grid),
            self.grid_height(grid),
        )
    }
    pub fn preferred_rows(&self) -> i32 {
        self.preferences.rows
    }
    pub fn preferred_rows_mut(&mut self) -> &mut i32 {
        &mut self.preferences.rows
    }
    pub fn preferred_columns(&self) -> i32 {
        self.preferences.columns
    }
    pub fn preferred_columns_mut(&mut self) -> &mut i32 {
        &mut self.preferences.columns
    }
    pub fn volume(&self) -> f32 {
        self.resources.sounds.volume
    }
    pub fn set_volume(&mut self, volume: f32) {
        self.resources.sounds.set_volume(volume)
    }

    pub fn new_button(&self, text: &str, anchor: Anchor) -> Button {
        new_button(text, anchor, self)
    }
    pub fn new_button_from_tr(&self, text_rect: TextRect) -> Button {
        text_rect.into()
    }
    pub fn new_text(&self, text: &str, anchor: Anchor) -> TextRect {
        self.new_text_size(text, anchor, 1.0)
    }
    pub fn new_text_size(&self, text: &str, anchor: Anchor, size_coef: f32) -> TextRect {
        new_text(text, anchor, size_coef, self)
    }

    pub fn small_triangle_half_width(&self) -> f32 {
        2.0 * self.cell_pad()
    }
    pub fn triangle_half_width(&self) -> f32 {
        4.0 * self.cell_pad()
    }
}

pub fn new_button(text: &str, anchor: Anchor, theme: &Theme) -> Button {
    let text_rect = new_text(text, anchor, 1.0, theme);
    text_rect.into()
}

pub fn labels_from_theme(theme: &Theme) -> LabelGroup {
    LabelGroup {
        font_size: 1.0,
        font: Some(theme.resources.font),
        alignment: Horizontal::Center,
        direction: Direction::Bottom,
        anchor: Default::default(),
        pad_x: None,
        pad_y: None,
        margin: 0.0,
    }
}
pub fn new_button_group(anchor: Anchor, theme: &Theme) -> ButtonGroup {
    new_button_group_generic(
        theme,
        LabelGroup {
            anchor,
            ..labels_from_theme(theme)
        },
    )
}
pub fn new_button_group_size(anchor: Anchor, theme: &Theme, font_size_coef: f32) -> ButtonGroup {
    new_button_group_generic(
        theme,
        LabelGroup {
            anchor,
            font_size: font_size_coef,
            ..labels_from_theme(theme)
        },
    )
}
pub fn new_button_group_direction(
    anchor: Anchor,
    theme: &Theme,
    direction: Direction,
) -> ButtonGroup {
    new_button_group_generic(
        theme,
        LabelGroup {
            anchor,
            direction,
            ..labels_from_theme(theme)
        },
    )
}
pub fn new_button_group_generic(theme: &Theme, labels: LabelGroup) -> ButtonGroup {
    let labels = LabelGroup {
        font_size: labels.font_size * theme.font_size(),
        ..labels
    };
    ButtonGroup::new_with_labels(labels)
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
#[allow(unused)]
pub fn new_text_group(anchor: Anchor, theme: &Theme) -> LabelGroup {
    LabelGroup::new_with_font(theme.font_size(), Some(theme.resources.font), anchor)
}
pub fn new_text_group_generic(anchor: Anchor, theme: &Theme, labels: LabelGroup) -> LabelGroup {
    LabelGroup {
        anchor,
        font_size: labels.font_size * theme.font_size(),
        ..labels
    }
}
pub fn render_text(text_rect: &TextRect, style: &StateStyle) {
    draw_rect(text_rect.rect(), style.bg_color);
    text_rect.render_default(style)
}
pub fn render_tooltip(text_rect: &TextRect, style: &StateStyle) {
    draw_rect(text_rect.rect(), style.bg_color);
    draw_rect_lines(text_rect.rect(), 2.0, style.border_color);
    text_rect.render_default(style)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{new_layout, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_WIDTH};

    #[test]
    fn test_basic_layout() {
        let layout = new_layout(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32);
        assert_eq!(
            layout,
            Layout {
                screen_width: 988.44446,
                screen_height: 556.0,
                font_size: 15.0,
                cell_width: 45.1,
                cell_height: 45.1,
                grid_pad: 30.0,
                cell_pad: 5.0,
                default_rows: 10,
                default_columns: 11
            }
        );
    }
    #[test]
    fn test_resize_grid() {
        let new_rows = 8;
        let new_columns = 9;
        let layout = new_layout(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32)
            .resize_grid(new_rows, new_columns);

        assert_eq!(
            layout,
            Layout {
                screen_width: 988.44446,
                screen_height: 556.0,
                font_size: 15.0,
                cell_width: 57.625,
                cell_height: 57.625,
                grid_pad: 30.0,
                cell_pad: 5.0,
                default_rows: 8,
                default_columns: 9
            }
        );
    }
    #[test]
    fn test_resize_screen() {
        let layout = new_layout(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32);
        let sw = DEFAULT_WINDOW_WIDTH as f32 * 2.0;
        let sh = DEFAULT_WINDOW_HEIGHT as f32 * 2.0;
        let layout = new_layout(sw, sh).resize_grid(layout.default_rows, layout.default_columns);

        assert_eq!(
            layout,
            Layout {
                screen_width: 1976.8889,
                screen_height: 1112.0,
                font_size: 22.5,
                cell_width: 95.45,
                cell_height: 95.45,
                grid_pad: 45.0,
                cell_pad: 7.5,
                default_rows: 10,
                default_columns: 11
            }
        );
    }
}
