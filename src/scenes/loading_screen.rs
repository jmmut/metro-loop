use macroquad::audio::{load_sound_from_bytes, Sound};
use crate::AnyError;
use crate::resource_loader::ResourceLoader;

pub async fn loading_screen() -> Result<(), AnyError> {
    // let mut sound_loader = ResourceLoader::new(&[
    //     include_bytes!("../../assets/sound/incorrect.wav"),
    //     include_bytes!("../../assets/sound/satisfied.wav"),
    //     include_bytes!("../../assets/sound/background.ogg"),
    //     include_bytes!("../../assets/sound/background_intro.ogg"),
    // ], load_sound_from_bytes);

    Ok(())
}