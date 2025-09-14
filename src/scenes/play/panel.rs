use juquad::widgets::Widget;
use juquad::draw::draw_rect;
use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::button::Button;
use juquad::widgets::text::TextRect;
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::math::{vec2, Rect, Vec2};
use crate::logic::constraints::Satisfaction;
use crate::render::{render_cross, render_tick};
use crate::{PANEL_BACKGROUND, SEE_SOLUTION_DURING_GAME, TEXT_STYLE};
use crate::theme::{new_button, new_text, new_text_group, render_button, render_text, render_tooltip, Theme};

pub struct Panel {
    pub rect: Rect,
    pub level_title: TextRect,
    pub next_game: Button,
    pub main_menu: Button,
    pub show_solution: Option<Button>,
}


impl Panel {
    pub fn new(panel_rect: Rect, theme: &Theme) -> Self {
        let anchor_point = vec2(
            panel_rect.x + panel_rect.w * 0.5,
            panel_rect.y + theme.button_pad(),
        );
        let half_pad = vec2(theme.cell_pad() * 0.5, 0.0);

        let level_name = theme.resources.level_history.current.to_string();
        let anchor_left = Anchor::top_right_v(anchor_point - half_pad);
        let level_title = new_text(&level_name, anchor_left, 1.0, theme);

        let anchor_right = Anchor::top_left_v(anchor_point + half_pad);
        let next_game = new_button("Next Game", anchor_right, &theme);

        let anchor_bottom = Anchor::bottom_center_v(vec2(
            panel_rect.x + panel_rect.w * 0.5,
            panel_rect.bottom() - theme.button_pad(),
        ));
        let main_menu = new_button("Main menu", anchor_bottom, theme);

        Self {
            rect: panel_rect,
            level_title,
            next_game,
            main_menu,
            show_solution: None,
        }
    }
    pub fn add_satisfaction(
        &mut self,
        satisfaction: &Satisfaction,
        theme: &Theme,
        show_solution: &mut bool,
    ) {
        let solved = satisfaction.success();
        let mut rect = if solved {
            let anchor = Anchor::below(self.filled_rect(), Horizontal::Center, theme.button_pad());
            let text = new_text(&"SOLVED!", anchor, 2.0, &theme);
            render_text(&text, &TEXT_STYLE);
            text.rect()
        } else {
            let texts_and_tooltips = [
                (&format!("{} incorrect rails", satisfaction.failing_rails), "some tooltip 1"),
                (&format!("{} cells to activate", satisfaction.cell_diff), "some tooltip 2"),
                (&format!("{} unreachable rails", satisfaction.unreachable_rails), "some tooltip 3"),
            ];
            let (texts, tooltips) = split_tuple(texts_and_tooltips);

            let anchor = Anchor::below(self.filled_rect(), Horizontal::Center, theme.button_pad());
            let labels = new_text_group(anchor, theme);
            let text_rects = labels.create(texts);

            let mut rect = Rect::default();
            let mouse_pos = Vec2::from(mouse_position());
            let clicked = is_mouse_button_pressed(MouseButton::Left);

            for (i, text_rect) in text_rects.into_iter().enumerate() {
                let icon_size = text_rect.rect().h;
                let anchor = Anchor::top_right_v(text_rect.rect().point());
                (if text_rect.text.starts_with('0') {
                    render_tick
                } else {
                    render_cross
                })(anchor, icon_size, theme);
                render_text(&text_rect, &TEXT_STYLE);
                rect = text_rect.rect();
                if clicked {
                    println!("{:?}", text_rect.rect());
                }
                if text_rect.rect().contains(mouse_pos) {
                    let anchor = Anchor::top_right_v(mouse_pos);
                    let tooltip = new_text(tooltips[i], anchor, 1.0, theme);
                    render_tooltip(&tooltip, &TEXT_STYLE);
                }
            }
            rect
        };
        self.show_solution = if solved || SEE_SOLUTION_DURING_GAME {
            rect.x = self.rect.x;
            rect.w = self.rect.w;
            let show_anchor = Anchor::below(rect, Horizontal::Center, theme.button_pad());
            let show_text = if *show_solution {
                "Hide solution"
            } else {
                "Show possible solution"
            };
            let show = new_button(show_text, show_anchor, &theme);
            Some(show)
        } else {
            None
        };
    }
    pub fn filled_rect(&self) -> Rect {
        let mut rect = self.rect;
        rect.h = self.next_game.rect().bottom() - rect.y;
        rect
    }
    
    pub fn interact(&mut self) {
        self.main_menu.interact();
        self.next_game.interact();
        if let Some(show) = &mut self.show_solution {
            show.interact();
        }
    }
    pub fn render_static(&self) {
        draw_rect(self.rect, PANEL_BACKGROUND);
        render_text(&self.level_title, &TEXT_STYLE);
    }
    pub fn render_interactive(&self) {
        render_button(&self.next_game);
        render_button(&self.main_menu);
        
        if let Some(show) = &self.show_solution {
            render_button(&show);
        }
    }
}
pub fn split_tuple<const N: usize, T: Copy, U: Copy>(array: [(T, U); N]) -> ([T; N], [U; N]) {
    let mut ts: [T; N] = [array[0].0; N];
    let mut us: [U; N] = [array[0].1; N];

    for (i, (t, u)) in array.into_iter().enumerate() {
        ts[i] = t;
        us[i] = u;
    }

    (ts, us)
}
