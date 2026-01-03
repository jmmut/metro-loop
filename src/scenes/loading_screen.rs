use crate::level_history::LevelHistory;
use crate::sound::Sounds;
use crate::theme::{new_text_unloaded, render_text, Preferences, Theme};
use crate::{
    new_layout, AnyError, BACKGROUND, DEFAULT_VOLUME, DISABLED_CELL, FAILING, RAIL, RAIL_BORDER,
    STYLE, SUCCESS, TRANSPARENT,
};
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::resource_loader::{Progress, ResourceLoader};
use juquad::widgets::anchor::Anchor;
use juquad::widgets::StateStyle;
use macroquad::audio::{load_sound_from_bytes, stop_sound, Sound};
use macroquad::color::DARKGRAY;
use macroquad::file::FileError;
use macroquad::math::Rect;
use macroquad::prelude::{
    clear_background, load_ttf_font_from_bytes, next_frame, screen_height, screen_width, Vec2,
};
use macroquad::text::Font;
use std::fmt::{Debug, Formatter};
use std::future::Future;

pub struct Resources {
    pub sounds: Sounds,
    pub font: Font,
    pub level_history: LevelHistory,
    // textures
}

trait ResourceLoaderTrait {
    fn get_progress(&self) -> Progress;
}
impl<In, Func: Fn(In) -> Fut, Fut: Future<Output = Result<Sound, FileError>>> ResourceLoaderTrait
    for ResourceLoader<In, Sound, FileError, Func, Fut>
{
    fn get_progress(&self) -> Progress {
        ResourceLoader::get_progress(self)
    }
}

impl<SoundLoader> TryFrom<Loading<SoundLoader>> for Resources {
    type Error = AnyError;

    fn try_from(value: Loading<SoundLoader>) -> Result<Self, Self::Error> {
        match value {
            Loading {
                sounds: LoadingSounds::Loaded(sounds),
                font: Some(font),
                level_history: Some(level_history),
            } => Ok(Resources {
                sounds,
                font,
                level_history,
            }),
            _ => Err(format!(
                "logic error: tried to convert to Resources from an incomplete Loading: {:?}",
                value
            )
            .into()),
        }
    }
}
impl Drop for Resources {
    fn drop(&mut self) {
        // in wasm, if you exit, apparently the sound keeps playing (?)
        for sound in self.sounds.list() {
            stop_sound(sound);
        }
    }
}

struct Loading<SoundLoader> {
    pub sounds: LoadingSounds<SoundLoader>,
    pub font: Option<Font>,
    pub level_history: Option<LevelHistory>,
}
impl<S> Debug for Loading<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
enum LoadingSounds<SoundLoader> {
    NotLoaded {
        loader_1: SoundLoader,
        loader_2: SoundLoader,
    },
    IntroLoaded(Sound, SoundLoader),
    Loaded(Sounds),
}
impl<S> Debug for LoadingSounds<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadingSounds::NotLoaded { .. } => {
                write!(f, "NotLoaded")
            }
            LoadingSounds::IntroLoaded(s, _) => {
                write!(f, "IntroLoaded ({:?}", s)
            }
            LoadingSounds::Loaded(s) => {
                write!(f, "Loaded ({:?})", s)
            }
        }
    }
}

impl<SoundLoader: ResourceLoaderTrait> LoadingSounds<SoundLoader> {
    pub fn add_intro(&mut self, intro: Sound) -> Result<(), AnyError> {
        let mut tmp = LoadingSounds::Loaded(Sounds::new(Vec::new()));
        std::mem::swap(&mut tmp, self);
        if let LoadingSounds::NotLoaded {
            loader_2: later_loader,
            ..
        } = tmp
        {
            *self = LoadingSounds::IntroLoaded(intro, later_loader);
            Ok(())
        } else {
            Err(format!("should be a LoadingSounds::NotLoaded but was {:?}", self).into())
        }
    }
    pub fn add_other(&mut self, mut other_sounds: Vec<Sound>) -> Result<(), AnyError> {
        if let LoadingSounds::IntroLoaded(intro, _) = self {
            other_sounds.push(*intro);
            let sounds = Sounds::new(other_sounds);
            *self = LoadingSounds::Loaded(sounds);
            Ok(())
        } else {
            Err(format!("should be a LoadingSounds::NotLoaded but was {:?}", self).into())
        }
    }
    pub fn get_total_to_load(&self) -> usize {
        match self {
            LoadingSounds::NotLoaded {
                loader_1: next_loader,
                loader_2: later_loader,
            } => {
                next_loader.get_progress().total_to_load + later_loader.get_progress().total_to_load
            }
            LoadingSounds::IntroLoaded(_, loader) => loader.get_progress().total_to_load,
            LoadingSounds::Loaded(_) => 0,
        }
    }
}
// impl PartialEq for LoadingSounds {
//     fn eq(&self, other: &Self) -> bool {
//         use LoadingSounds::*;
//         match (self, other) {
//             (NotLoaded, NotLoaded) | (IntroLoaded(_), IntroLoaded(_)) | (Loaded(_), Loaded(_)) => true,
//             (_, _) => false,
//         }
//     }
// }

impl<SoundLoader> TryFrom<LoadingSounds<SoundLoader>> for Sounds {
    type Error = AnyError;

    fn try_from(loading: LoadingSounds<SoundLoader>) -> Result<Self, Self::Error> {
        match loading {
            LoadingSounds::Loaded(sounds) => Ok(sounds),
            LoadingSounds::NotLoaded { .. } | LoadingSounds::IntroLoaded(_, _) => Err(format!(
                "logic error: tried to convert to Sounds from an incomplete LoadingSounds: {:?}",
                loading
            )
            .into()),
        }
    }
}

const INCORRECT: &[u8] = include_bytes!("../../assets/sound/incorrect.wav");
const BACKGROUND_INTRO: &[u8] = include_bytes!("../../assets/sound/background_intro.ogg");
const SATISFIED: &[u8] = include_bytes!("../../assets/sound/satisfied.wav");
const BACKGROUND_SONG: &[u8] = include_bytes!("../../assets/sound/background.ogg");

pub async fn loading_screen(
    section: i32,
    level: i32,
    sound_enabled: bool,
) -> Result<Theme, AnyError> {
    let loading_sounds = if sound_enabled {
        let sound_loader_1 =
            ResourceLoader::<_, Sound, _, _, _>::new(load_sound_from_bytes, vec![BACKGROUND_INTRO]);
        let sound_loader_2 = ResourceLoader::<_, Sound, _, _, _>::new(
            load_sound_from_bytes,
            vec![INCORRECT, SATISFIED, BACKGROUND_SONG],
        );
        LoadingSounds::NotLoaded {
            loader_1: sound_loader_1,
            loader_2: sound_loader_2,
        }
    } else {
        LoadingSounds::Loaded(Sounds::new(Vec::new()))
    };

    let (sw, sh) = (screen_width(), screen_height());
    let layout = new_layout(sw, sh);
    let preferences = Preferences::new();
    let total_progress = loading_sounds.get_total_to_load()
        + 1 // font
        + 1 // levels
        ;
    let mut loading = Loading {
        sounds: loading_sounds,
        font: None,
        level_history: None,
    };
    let mut progress = 0;
    loop {
        let mut stage_progress = 0;
        if let LoadingSounds::NotLoaded { loader_1, .. } = &mut loading.sounds {
            let sound_progress = loader_1.get_progress();
            if let Some(loaded) = loader_1.get_resources()? {
                let intro = loaded[0];
                Sounds::play_looped(intro, DEFAULT_VOLUME);
                loading.sounds.add_intro(intro)?;
                progress += sound_progress.total_to_load;
            } else {
                stage_progress = sound_progress.loaded;
            }
        } else if let LoadingSounds::IntroLoaded(intro, sound_loader_2) = &mut loading.sounds {
            let sound_progress = sound_loader_2.get_progress();
            if let Some(loaded) = sound_loader_2.get_resources()? {
                Sounds::play_looped(loaded[2], DEFAULT_VOLUME);
                Sounds::stop(*intro);
                loading.sounds.add_other(loaded)?;
                progress += sound_progress.total_to_load;
            } else {
                stage_progress = sound_progress.loaded;
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
            let resources: Resources = loading.try_into()?;
            return Ok(Theme {
                resources,
                layout,
                preferences,
            });

            // uncomment to see loading screen infinitely
            // resources.sounds.stop_background();
            // progress = 0;
            // loading = Loading {
            //     sounds: LoadingSounds::NotLoaded,
            //     font: None,
            //     level_history: None,
            // };
        }
        stage_progress += progress;
        // println!(
        //     "stage_progress: {}, progress: {}, total: {}",
        //     stage_progress, progress, total_progress
        // );

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
        draw_rect_lines(outer_rect, 2.0, RAIL_BORDER);
        // bar_rect.y += bar_rect.h;
        draw_rect(bar_rect, FAILING);
        bar_rect.w = stage_progress as f32 * rect.rect.w / total_progress as f32;
        draw_rect(bar_rect, SUCCESS);
        bar_rect.w = rect.rect.w;
        draw_rect_lines(bar_rect, 2.0, RAIL_BORDER);
        render_text(
            &rect,
            &StateStyle {
                bg_color: TRANSPARENT,
                text_color: STYLE.at_rest.text_color,
                border_color: DARKGRAY,
            },
        );
        next_frame().await
    }
}
