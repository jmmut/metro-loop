use crate::logic::constraints::Satisfaction;
use crate::render::{render_cross, render_tick};
use crate::theme::{
    labels_from_theme, new_button, new_text, new_text_group_generic, render_button, render_text,
    Theme,
};
use crate::{PANEL_BACKGROUND, SEE_SOLUTION_DURING_GAME, TEXT_STYLE};
use juquad::draw::draw_rect;
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
    pub next_game: Button,
    pub main_menu: Button,
    pub show_solution: Option<Button>,
    satisfaction: SatisfactionPanel,
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

    pub fn interact(&mut self, theme: &Theme) {
        self.main_menu.interact();
        self.next_game.interact();
        if let Some(show) = &mut self.show_solution {
            show.interact();
        }
        self.satisfaction.interact(theme);
    }
    pub fn render_static(&self, theme: &Theme) {
        draw_rect(self.rect, PANEL_BACKGROUND);
        render_text(&self.level_title, &TEXT_STYLE);
        self.satisfaction.render_static(theme);
    }
    pub fn render_interactive(&self) {
        render_button(&self.next_game);
        render_button(&self.main_menu);

        if let Some(show) = &self.show_solution {
            render_button(&show);
        }
        self.satisfaction.render_interactive();
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

impl SatisfactionPanel {
    pub fn new(satisfaction: Satisfaction, previous_rect: Rect, theme: &Theme) -> Self {
        if satisfaction.success() {
            let anchor = Anchor::below(previous_rect, Horizontal::Center, theme.button_pad());
            let text = new_text(&"SOLVED!", anchor, 2.0, &theme);
            Self::Solved { text }
        } else {
            #[rustfmt::skip]
            let texts_and_tooltips = [
                ((&satisfaction.stations.format("satisfied stations"), satisfaction.stations.success()), "some tooltip 1"),
                ((&satisfaction.cell_count.format("active cells"), satisfaction.cell_count.success()), "some tooltip 2"),
                ((&satisfaction.reachable.format("reachable rails"), satisfaction.reachable.success()), "some tooltip 3"),
            ];
            let (texts_success, tooltips) = split_tuple(texts_and_tooltips);
            let (texts, successes) = split_tuple(texts_success);

            let anchor = Anchor::below(previous_rect, Horizontal::Center, theme.button_pad());
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
                texts: text_rects.into(),
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
                    if text.rect().contains(mouse_pos) {
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
                for (i, text_rect) in texts.iter().enumerate() {
                    let icon_size = text_rect.rect().h;
                    let anchor = Anchor::top_right_v(text_rect.rect().point());
                    (if successes[i] {
                        render_tick
                    } else {
                        render_cross
                    })(anchor, icon_size, theme);
                    render_text(&text_rect, &TEXT_STYLE);
                }
            }
            Self::Unknown => {}
        }
    }
    pub fn render_interactive(&self) {
        match self {
            Self::Solved { text } => render_text(&text, &TEXT_STYLE),
            Self::Unsolved { tooltips, .. } => {
                for tooltip in tooltips {
                    match tooltip {
                        Tooltip::Text(_) => {}
                        Tooltip::Renderable(_tooltip) => {
                            // render_tooltip(&tooltip, &TEXT_STYLE);
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
            SatisfactionPanel::Unsolved { texts, .. } => texts
                .first()
                .unwrap()
                .rect()
                .combine_with(texts.last().unwrap().rect()),
            SatisfactionPanel::Unknown => panic!("logic error: should be unreachable"),
        }
    }

    fn rect_mut(&mut self) -> &mut Rect {
        unimplemented!()
    }
}
