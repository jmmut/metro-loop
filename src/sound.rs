use crate::DEFAULT_VOLUME;
use macroquad::audio::{play_sound, set_sound_volume, stop_sound, PlaySoundParams, Sound};

#[derive(Debug)]
pub struct Sounds {
    available: AvailableSounds,
    pub volume: f32,
}

#[derive(Debug)]
struct AvailableSounds {
    pub _sound_incorrect: SoundConfig,
    pub sound_correct: SoundConfig,
    pub music_background: SoundConfig,
    pub music_background_intro: SoundConfig,
}

#[derive(Debug, Copy, Clone)]
struct SoundConfig {
    sound: Sound,
    volume_coef: f32,
}
impl SoundConfig {
    pub fn new(sound: Sound, volume_coef: f32) -> Self {
        Self { sound, volume_coef }
    }
}

impl Sounds {
    pub fn new(sounds: Vec<Sound>) -> Self {
        let n = size_of::<AvailableSounds>() / size_of::<SoundConfig>();
        assert_eq!(
            sounds.len(),
            n,
            "{} sounds were passed, but expected {}",
            sounds.len(),
            n
        );

        Self {
            available: AvailableSounds {
                _sound_incorrect: SoundConfig::new(sounds[0], 0.05),
                sound_correct: SoundConfig::new(sounds[1], 0.05),
                music_background: SoundConfig::new(sounds[2], 1.0),
                music_background_intro: SoundConfig::new(sounds[3], 1.0),
            },
            volume: DEFAULT_VOLUME,
        }
    }
    pub fn list(&self) -> Vec<Sound> {
        self.list_volumes()
            .into_iter()
            .map(|sound_config| sound_config.sound)
            .collect()
    }
    fn list_volumes(&self) -> Vec<SoundConfig> {
        vec![
            self.available.sound_correct,
            self.available._sound_incorrect,
            self.available.music_background,
            self.available.music_background_intro,
        ]
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
        for SoundConfig { sound, volume_coef } in self.list_volumes() {
            set_sound_volume(sound, self.volume * volume_coef);
        }
    }

    pub fn play_correct(&self) {
        self.play_config(self.available.sound_correct)
    }
    pub fn play_background(&self) {
        self.play_config_looped(self.available.music_background)
    }
    pub fn play_background_intro(&self) {
        self.play_config_looped(self.available.music_background_intro)
    }

    fn play_config(&self, sound_config: SoundConfig) {
        Self::play(sound_config.sound, self.volume * sound_config.volume_coef)
    }
    fn play_config_looped(&self, sound_config: SoundConfig) {
        Self::play_looped(sound_config.sound, self.volume * sound_config.volume_coef)
    }

    pub fn play(sound: Sound, volume: f32) {
        Self::play_params(sound, volume, false)
    }
    pub fn play_looped(sound: Sound, volume: f32) {
        Self::play_params(sound, volume, true)
    }
    fn play_params(sound: Sound, volume: f32, looped: bool) {
        play_sound(sound, PlaySoundParams { looped, volume });
    }
    pub fn stop(sound: Sound) {
        stop_sound(sound)
    }
}
