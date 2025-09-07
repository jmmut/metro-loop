use macroquad::audio::Sound;

#[derive(Debug)]
pub struct Sounds {
    pub _sound_incorrect: Sound,
    pub sound_correct: Sound,
    pub music_background: Sound,
    pub music_background_intro: Sound,
}

impl Sounds {
    pub fn new(sounds: Vec<Sound>) -> Self {
        let n = size_of::<Sounds>() / size_of::<Sound>();
        assert_eq!(
            sounds.len(),
            n,
            "{} sounds were passed, but expected {}",
            sounds.len(),
            n
        );

        Self {
            _sound_incorrect: sounds[0],
            sound_correct: sounds[1],
            music_background: sounds[2],
            music_background_intro: sounds[3],
        }
    }
    pub fn list(&self) -> Vec<Sound> {
        vec![
            self.sound_correct,
            self._sound_incorrect,
            self.music_background,
            self.music_background_intro,
        ]
    }
}
