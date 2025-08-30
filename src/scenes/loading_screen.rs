use crate::render::{new_text, render_text};
use crate::sound::Sounds;
use crate::{AnyError, BACKGROUND, CELL_PAD, ENABLED_CELL, GRID_PAD, STYLE};
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::resource_loader::ResourceLoader;
use juquad::widgets::anchor::Anchor;
use macroquad::audio::{load_sound_from_bytes, Sound};
use macroquad::color::Color;
use macroquad::math::Rect;
use macroquad::prelude::{clear_background, next_frame, screen_height, screen_width, Vec2};

pub struct Resources {
    pub sounds: Sounds,
    // font
    // textures
}

pub async fn loading_screen() -> Result<Resources, AnyError> {
    let mut sound_loader = ResourceLoader::<_, Sound, _, _, _>::new(
        load_sound_from_bytes,
        &[
            include_bytes!("../../assets/sound/incorrect.wav"),
            include_bytes!("../../assets/sound/satisfied.wav"),
            include_bytes!("../../assets/sound/background.ogg"),
            include_bytes!("../../assets/sound/background_intro.ogg"),
        ],
    );

    let mut resources = None;
    loop {
        let (sw, sh) = (screen_width(), screen_height());
        let panel = Rect::new(GRID_PAD, GRID_PAD, sw - GRID_PAD * 2.0, sh - GRID_PAD * 2.0);
        match resources {
            None => {
                clear_background(Color::new(0.0, 0.0, 0.0, 0.0));
                draw_rect(panel, STYLE.pressed.bg_color);
                let center = Vec2::new(sw * 0.5, sh * 0.5);
                let rect = new_text("Loading...", Anchor::center_v(center), 2.0);
                render_text(&rect, &STYLE.pressed);
                let mut bar_rect = rect.rect;
                bar_rect.y += bar_rect.h;
                let progress = sound_loader.get_progress();
                bar_rect.w = progress.loaded as f32 * rect.rect.w / progress.total_to_load as f32;
                draw_rect(bar_rect, ENABLED_CELL);
                bar_rect.w = rect.rect.w;
                draw_rect_lines(bar_rect, CELL_PAD * 2.0, BACKGROUND);

                if let Some(loaded) = sound_loader.get_resources()? {
                    resources = Some(loaded);
                }
            }
            Some(sounds) => {
                return Ok(Resources {
                    sounds: Sounds::new(sounds),
                });
                // resources = None; // to see the loading screen in loop
            }
        }
        next_frame().await
    }
}
