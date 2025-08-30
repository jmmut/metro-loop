use crate::render::{new_text, render_text};
use crate::sound::Sounds;
use crate::{
    AnyError, BACKGROUND, CELL_PAD, DISABLED_CELL, ENABLED_CELL, GRID_PAD, RAIL, STYLE,
    TRANSPARENT, TRIANGLE_BORDER,
};
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::resource_loader::ResourceLoader;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::StateStyle;
use macroquad::audio::{load_sound_from_bytes, Sound};
use macroquad::color::DARKGRAY;
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
                clear_background(BACKGROUND);
                draw_rect(panel, DISABLED_CELL);
                let center = Vec2::new(sw * 0.5, sh * 0.5);
                let rect = new_text("Loading...", Anchor::center_v(center), 2.0);
                let mut bar_rect = rect.rect;

                let outer_rect = Rect::new(
                    bar_rect.x - CELL_PAD,
                    bar_rect.y - CELL_PAD,
                    bar_rect.w + 2.0 * CELL_PAD,
                    bar_rect.h + 2.0 * CELL_PAD,
                );
                draw_rect(outer_rect, RAIL);
                draw_rect_lines(outer_rect, 2.0, TRIANGLE_BORDER);
                // bar_rect.y += bar_rect.h;
                draw_rect(bar_rect, DISABLED_CELL);
                let progress = sound_loader.get_progress();
                bar_rect.w = progress.loaded as f32 * rect.rect.w / progress.total_to_load as f32;
                draw_rect(bar_rect, ENABLED_CELL);
                bar_rect.w = rect.rect.w;
                draw_rect_lines(bar_rect, 2.0, TRIANGLE_BORDER);
                render_text(
                    &rect,
                    &StateStyle {
                        bg_color: TRANSPARENT,
                        text_color: STYLE.pressed.text_color,
                        border_color: DARKGRAY,
                    },
                );

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
