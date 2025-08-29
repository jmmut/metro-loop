use juquad::draw::draw_rect;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::text::TextRect;
use macroquad::audio::{play_sound, play_sound_once, PlaySoundParams, Sound};
use macroquad::camera::{set_camera, set_default_camera, Camera2D};
use macroquad::color::{Color, WHITE};
use macroquad::input::{
    is_key_pressed, is_mouse_button_pressed, is_mouse_button_released, mouse_position, KeyCode,
    MouseButton,
};
use macroquad::math::{ivec2, vec2, Rect, Vec2};
use macroquad::miniquad::date::now;
use macroquad::miniquad::FilterMode;
use macroquad::prelude::{
    clear_background, draw_texture_ex, get_fps, next_frame, screen_height, screen_width, Conf,
    DrawTextureParams,
};
use macroquad::rand::srand;
use metro_loop::{
    scenes, AnyError, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE, DEFAULT_WINDOW_WIDTH,
};

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let seed = now() as u64;
    srand(seed);
    // let mut audio_ctx = AudioContext::new();
    // let sound = quad_snd::Sound::load(&audio_ctx, include_bytes!("../assets/sound/background_intro.ogg"));

    // let sound_incorrect = load_sound_from_bytes(include_bytes!("../assets/sound/incorrect.wav"));
    // let sound_correct = load_sound_from_bytes(include_bytes!("../assets/sound/satisfied.wav"));
    // let music_background = load_sound_from_bytes(include_bytes!("../assets/sound/background.ogg"));
    // let music_background_intro = load_sound_from_bytes(include_bytes!("../assets/sound/background_intro.ogg"));
    // let sound_incorrect = sound_incorrect.?;
    // let sound_correct = sound_correct.await?;
    // let music_background = music_background.await?;
    // let music_background_intro = music_background_intro.await?;

    scenes::play().await?;
    Ok(())
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}
