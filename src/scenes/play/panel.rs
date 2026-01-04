use crate::level_history::GameTrack;
use crate::logic::constraints::Satisfaction;
use crate::render::{
    draw_blockade, draw_line_thickness, draw_rail, draw_station, render_cross, render_tick,
};
use crate::theme::{
    labels_from_theme, new_button, new_text, new_text_group_generic, render_button, render_text,
    render_tooltip, Theme,
};
use crate::{
    BACKGROUND, ENABLED_CELL, PANEL_BACKGROUND, SEE_SOLUTION_DURING_GAME, SUCCESS, SUCCESS_DARK,
    TEXT_STYLE, TRIANGLE, TRIANGLE_BORDER,
};
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::lazy::add_contour;
use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::button::Button;
use juquad::widgets::button_group::LabelGroup;
use juquad::widgets::text::TextRect;
use juquad::widgets::Widget;
use macroquad::input::mouse_position;
use macroquad::math::{vec2, Rect, Vec2};

pub struct Panel {
    pub rect: Rect,
    pub level_title: TextRect,
    pub restart_game: Button,
    pub next_game: Button,
    pub main_menu: Button,
    pub show_solution: Option<Button>,
    satisfaction: SatisfactionPanel,
}

impl Panel {
    pub fn new(panel_rect: Rect, theme: &Theme, game_track: &GameTrack) -> Self {
        let anchor_point = vec2(
            panel_rect.x + panel_rect.w * 0.5,
            panel_rect.y + theme.button_pad(),
        );
        let _half_pad = vec2(theme.cell_pad() * 0.5, 0.0);

        let level_name = game_track.current.to_string();
        let anchor_name = Anchor::top_center_v(anchor_point);
        let level_title = new_text(&level_name, anchor_name, 1.0, theme);
        let below_title = vec2(
            level_title.rect.center().x,
            level_title.rect.bottom() + theme.cell_pad(),
        );

        let anchor_next = Anchor::top_center_v(below_title);
        let restart_game = new_button("RESTART", anchor_next, &theme);

        let anchor_next = Anchor::below(restart_game.rect(), Horizontal::Center, theme.cell_pad());
        let next_game = new_button("NEXT", anchor_next, &theme);

        let anchor_bottom = Anchor::bottom_center_v(vec2(
            panel_rect.x + panel_rect.w * 0.5,
            panel_rect.bottom() - theme.button_pad(),
        ));
        let main_menu = new_button("MENU", anchor_bottom, theme);

        Self {
            rect: panel_rect,
            level_title,
            restart_game,
            next_game,
            main_menu,
            show_solution: None,
            satisfaction: SatisfactionPanel::Unknown,
        }
    }
    pub fn add_satisfaction(
        &mut self,
        satisfaction: &Satisfaction,
        theme: &Theme,
        show_solution: &mut bool,
    ) {
        let satisfaction_panel = SatisfactionPanel::new(*satisfaction, self.filled_rect(), theme);
        self.satisfaction = satisfaction_panel;
        let mut rect = self.satisfaction.rect();
        self.show_solution = if satisfaction.success() || SEE_SOLUTION_DURING_GAME {
            rect.x = self.rect.x;
            rect.w = self.rect.w;
            let show_anchor = Anchor::below(rect, Horizontal::Center, theme.button_pad());
            let show_text = if *show_solution {
                "HIDE SOLUTION"
            } else if satisfaction.success() {
                "SHOW POSSIBLE SOLUTION"
            } else {
                "GIVE UP"
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

    pub fn interact(&mut self, theme: &Theme) {
        for b in self.buttons_mut() {
            b.interact();
        }
        self.satisfaction.interact(theme);
    }
    pub fn render_static(&self, theme: &Theme) {
        draw_rect(self.rect, PANEL_BACKGROUND);
        render_text(&self.level_title, &TEXT_STYLE);
        self.satisfaction.render_static(theme);
    }
    pub fn render_interactive(&self) {
        for b in self.buttons() {
            render_button(b);
        }
        self.satisfaction.render_interactive();
    }
    pub fn buttons(&self) -> Vec<&Button> {
        let mut buttons = vec![&self.main_menu, &self.restart_game, &self.next_game];
        if let Some(button) = self.show_solution.as_ref() {
            buttons.push(button)
        }
        buttons
    }
    pub fn buttons_mut(&mut self) -> Vec<&mut Button> {
        let mut buttons = vec![
            &mut self.main_menu,
            &mut self.restart_game,
            &mut self.next_game,
        ];
        if let Some(button) = self.show_solution.as_mut() {
            buttons.push(button)
        }
        buttons
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

enum SatisfactionPanel {
    Solved {
        text: TextRect,
    },
    Unsolved {
        // satisfaction: Satisfaction,
        // previous_rect: Rect,
        texts: Vec<TextRect>,
        successes: Vec<bool>,
        tooltips: Vec<Tooltip>,
    },
    Unknown,
}

enum Tooltip {
    Text(String),
    Renderable(TextRect),
}
impl Tooltip {
    pub fn text(&self) -> &str {
        match self {
            Tooltip::Text(t) => t,
            Tooltip::Renderable(t) => &t.text,
        }
    }
}

pub fn station_render() -> impl Fn(&Theme) {
    |_theme| {}
}

impl SatisfactionPanel {
    pub fn new(satisfaction: Satisfaction, previous_rect: Rect, theme: &Theme) -> Self {
        if satisfaction.success() {
            let anchor = Anchor::below(previous_rect, Horizontal::Center, theme.button_pad());
            let text = new_text(&"SOLVED!", anchor, 2.0, &theme);
            Self::Solved { text }
        } else {
            #[rustfmt::skip]
            let texts_and_tooltips = [
                ((&satisfaction.stations.format(), satisfaction.stations.success()), "Satisfied bridges and stations"),
                ((&satisfaction.cell_count.format(), satisfaction.cell_count.success()), "Active blocks"),
                ((&satisfaction.reachable.format(), satisfaction.reachable.success()), "Reachable rails"),
            ];
            let (texts_success, tooltips) = split_tuple(texts_and_tooltips);
            let (texts, successes) = split_tuple(texts_success);

            let anchor_point = vec2(
                previous_rect.center().x,
                previous_rect.bottom() + theme.button_pad(),
            );
            let anchor = Anchor::top_left_v(anchor_point);
            let labels = new_text_group_generic(
                anchor,
                theme,
                LabelGroup {
                    alignment: Horizontal::Left,
                    ..labels_from_theme(theme)
                },
            );
            let text_rects = labels.create(texts);

            Self::Unsolved {
                texts: text_rects
                    .into_iter()
                    .map(|mut t| {
                        t.rect.x += t.rect.h * 0.5;
                        t
                    })
                    .collect(),
                successes: successes.into(),
                tooltips: tooltips
                    .into_iter()
                    .map(|t| Tooltip::Text(t.to_string()))
                    .collect(),
            }
        }
    }
    pub fn interact(&mut self, theme: &Theme) {
        match self {
            Self::Unsolved {
                texts, tooltips, ..
            } => {
                let mouse_pos = Vec2::from(mouse_position());
                for (i, text) in texts.iter().enumerate() {
                    if text.rect().contains(mouse_pos)
                        || Self::get_icon_rect(text).contains(mouse_pos)
                    {
                        let anchor = Anchor::bottom_right_v(mouse_pos);
                        let tooltip = new_text(tooltips[i].text(), anchor, 1.0, theme);
                        tooltips[i] = Tooltip::Renderable(tooltip);
                    } else {
                        match &tooltips[i] {
                            Tooltip::Text(_) => {}
                            Tooltip::Renderable(text_rect) => {
                                tooltips[i] = Tooltip::Text(text_rect.text.to_string());
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    pub fn render_static(&self, theme: &Theme) {
        match self {
            Self::Solved { text } => render_text(&text, &TEXT_STYLE),
            Self::Unsolved {
                texts, successes, ..
            } => {
                let mut cross_tick_rects = Vec::new();
                for (i, text_rect) in texts.iter().enumerate() {
                    let icon_rect = Self::get_icon_rect(text_rect);
                    cross_tick_rects.push((if successes[i] {
                        render_tick
                    } else {
                        render_cross
                    })(icon_rect, theme));
                    render_text(&text_rect, &TEXT_STYLE);
                }
                assert_eq!(cross_tick_rects.len(), 3);
                let icon_size = cross_tick_rects[0].size();
                let margin_x = vec2(icon_size.x * 0.375, 0.0);
                let anchor = Anchor::top_right_v(cross_tick_rects[0].point() - margin_x);
                let icon_rect = anchor.get_rect(icon_size);
                // let icon_rect = add_contour(icon_rect, Vec2::splat(theme.cell_pad()));
                let width = vec2(icon_rect.w, 0.0);
                let start = icon_rect.center() - width * 0.5;
                draw_rail(start, start + width, theme, true);
                draw_station(theme, true, SUCCESS, SUCCESS_DARK, start, width, false);
                let start_2 = start - width;
                draw_line_thickness(start_2, start, theme.cell_pad(), BACKGROUND);
                draw_blockade(
                    theme,
                    true,
                    SUCCESS,
                    SUCCESS_DARK,
                    start_2,
                    width,
                    false,
                    false,
                    false,
                );

                let anchor = Anchor::top_right_v(cross_tick_rects[1].point() - margin_x);
                let icon_rect = anchor.get_rect(icon_size);
                // let icon_rect = add_contour(icon_rect, -Vec2::splat(theme.cell_pad()));
                draw_rect(icon_rect, TRIANGLE);
                draw_rect_lines(icon_rect, 2.0, TRIANGLE_BORDER);
                let cell_rect = add_contour(icon_rect, -Vec2::splat(theme.cell_pad()));
                draw_rect(cell_rect, ENABLED_CELL);
                draw_rect_lines(cell_rect, 2.0, TRIANGLE_BORDER);

                let anchor = Anchor::top_right_v(cross_tick_rects[2].point() - margin_x);
                let icon_rect = anchor.get_rect(icon_size);
                let length = vec2(icon_rect.w, 0.0);
                let end = icon_rect.center() + length * 0.5;
                draw_rail(end - length, end, theme, true);
            }
            Self::Unknown => {}
        }
    }

    fn get_icon_rect(text_rect: &TextRect) -> Rect {
        let icon_size = text_rect.rect().h;
        let anchor = Anchor::top_right_v(text_rect.rect().point());
        let icon_rect = anchor.get_rect(vec2(icon_size, icon_size));
        icon_rect
    }

    pub fn render_interactive(&self) {
        match self {
            Self::Solved { .. } => {}
            Self::Unsolved { tooltips, .. } => {
                for tooltip in tooltips {
                    match tooltip {
                        Tooltip::Text(_) => {}
                        Tooltip::Renderable(tooltip) => {
                            render_tooltip(&tooltip, &TEXT_STYLE);
                        }
                    }
                }
            }
            Self::Unknown => {}
        }
    }
}

impl Widget for SatisfactionPanel {
    fn rect(&self) -> Rect {
        match self {
            SatisfactionPanel::Solved { text } => text.rect(),
            SatisfactionPanel::Unsolved { texts, .. } =>
            //Rect::default(),
            {
                texts
                    .first()
                    .unwrap()
                    .rect()
                    .combine_with(texts.last().unwrap().rect())
            }
            SatisfactionPanel::Unknown => panic!("logic error: should be unreachable"),
        }
    }

    fn set_rect(&mut self, _rect: Rect) {
        unimplemented!()
    }
}
