use crate::level_history::LevelHistory;
use crate::sound::Sounds;
use crate::theme::{new_text_unloaded, render_text, Theme};
use crate::{
    new_layout, AnyError, BACKGROUND, DEFAULT_VOLUME, DISABLED_CELL, ENABLED_CELL, RAIL, STYLE,
    TRANSPARENT, TRIANGLE_BORDER,
};
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::resource_loader::ResourceLoader;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::StateStyle;
use macroquad::audio::{load_sound_from_bytes, play_sound, stop_sound, PlaySoundParams, Sound};
use macroquad::color::DARKGRAY;
use macroquad::math::Rect;
use macroquad::prelude::{
    clear_background, load_ttf_font_from_bytes, next_frame, screen_height, screen_width, Vec2,
};
use macroquad::text::Font;

pub struct Resources {
    pub sounds: Sounds,
    pub font: Font,
    pub level_history: LevelHistory,
    // textures
}

impl TryFrom<Loading> for Resources {
    type Error = AnyError;

    fn try_from(value: Loading) -> Result<Self, Self::Error> {
        match value {
            Loading {
                sound_intro: Some(sound_intro),
                sounds_2: Some(mut other_sounds),
                font: Some(font),
                level_history: Some(level_history),
            } => {
                other_sounds.push(sound_intro);
                let sounds = Sounds::new(other_sounds);
                Ok(Resources {
                    sounds,
                    font,
                    level_history,
                })
            }
            _ => Err(format!(
                "logic error: tried to convert to Resources from an incomplete Loading: {:?}",
                value
            )
            .into()),
        }
    }
}

#[derive(Debug)]
pub struct Loading {
    pub sound_intro: Option<Sound>,
    pub sounds_2: Option<Vec<Sound>>,
    pub font: Option<Font>,
    pub level_history: Option<LevelHistory>,
}

const LOOPED_SOUND: PlaySoundParams = PlaySoundParams {
    looped: true,
    volume: DEFAULT_VOLUME,
};

pub async fn loading_screen(section: i32, level: i32) -> Result<Theme, AnyError> {
    let mut sound_loader_1 = ResourceLoader::<_, Sound, _, _, _>::new(
        load_sound_from_bytes,
        &[include_bytes!("../../assets/sound/background_intro.ogg")],
    );
    let mut sound_loader_2 = ResourceLoader::<_, Sound, _, _, _>::new(
        load_sound_from_bytes,
        &[
            include_bytes!("../../assets/sound/incorrect.wav"),
            include_bytes!("../../assets/sound/satisfied.wav"),
            include_bytes!("../../assets/sound/background.ogg"),
        ],
    );

    let (sw, sh) = (screen_width(), screen_height());
    let layout = new_layout(sw, sh);
    let mut loading = Loading {
        sound_intro: None,
        sounds_2: None,
        font: None,
        level_history: None,
    };
    let total_progress = sound_loader_1.get_progress().total_to_load
        + sound_loader_2.get_progress().total_to_load
        + 1 // font
        + 1 // levels
        ;
    let mut progress = 0;
    loop {
        if loading.sound_intro.is_none() {
            let sound_progress = sound_loader_1.get_progress();
            if let Some(loaded) = sound_loader_1.get_resources()? {
                loading.sound_intro = Some(loaded[0]);
                play_sound(*loading.sound_intro.as_ref().unwrap(), LOOPED_SOUND);
                progress = sound_progress.total_to_load;
            } else {
                progress = sound_progress.loaded;
            }
        } else if loading.sounds_2.is_none() {
            let sound_progress = sound_loader_2.get_progress();
            if let Some(loaded) = sound_loader_2.get_resources()? {
                loading.sounds_2 = Some(loaded);
                play_sound(loading.sounds_2.as_ref().unwrap()[2], LOOPED_SOUND);
                stop_sound(*loading.sound_intro.as_ref().unwrap());
                progress = sound_progress.total_to_load;
            } else {
                progress = sound_progress.loaded;
            }
        } else if loading.font.is_none() {
            let font_bytes = include_bytes!("../../assets/fonts/Saira-Regular.ttf");
            let font = load_ttf_font_from_bytes(font_bytes).unwrap();
            loading.font = Some(font);
            progress += 1;
        } else if loading.level_history.is_none() {
            loading.level_history = Some(LevelHistory::new(section, level)?);
            progress += 1;
        } else {
            let resources = loading.try_into()?;
            return Ok(Theme { resources, layout });
        }
        // resources = None; // to see the loading screen in loop

        let (sw, sh) = (screen_width(), screen_height());
        let grid_pad = layout.grid_pad();
        let panel = Rect::new(grid_pad, grid_pad, sw - grid_pad * 2.0, sh - grid_pad * 2.0);
        clear_background(BACKGROUND);
        draw_rect(panel, DISABLED_CELL);
        let center = Vec2::new(sw * 0.5, sh * 0.5);
        let rect = new_text_unloaded("Loading...", Anchor::center_v(center), 2.0, &layout);
        let mut bar_rect = rect.rect;

        let outer_rect = Rect::new(
            bar_rect.x - layout.cell_pad(),
            bar_rect.y - layout.cell_pad(),
            bar_rect.w + 2.0 * layout.cell_pad(),
            bar_rect.h + 2.0 * layout.cell_pad(),
        );
        draw_rect(outer_rect, RAIL);
        draw_rect_lines(outer_rect, 2.0, TRIANGLE_BORDER);
        // bar_rect.y += bar_rect.h;
        draw_rect(bar_rect, DISABLED_CELL);
        bar_rect.w = progress as f32 * rect.rect.w / total_progress as f32;
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
        next_frame().await
    }
}
