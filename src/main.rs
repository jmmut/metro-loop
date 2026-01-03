use macroquad::miniquad::date::now;
use macroquad::prelude::{next_frame, Conf};
use macroquad::rand::srand;
use metro_loop::{
    scenes, AnyError, NextStage, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE, DEFAULT_WINDOW_WIDTH,
    STARTING_LEVEL, STARTING_SECTION,
};
use metro_loop::level_history::GameTrack;

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let seed = now() as u64;
    srand(seed);

    let args = parse_args()?;
    let mut theme = scenes::loading_screen(args.sound_enabled).await?;
    let mut game_track = GameTrack::new(args.section, args.level, &theme.resources.levels)?;
    let mut next_stage = NextStage::MainMenu;
    loop {
        next_stage = match next_stage {
            NextStage::MainMenu => scenes::main_menu(&mut theme).await?,
            NextStage::Campaign => scenes::play(&mut theme, &mut game_track).await?,
            NextStage::Options => scenes::options(&mut theme).await?,
            NextStage::Quit => return Ok(()),
        };
        next_frame().await
    }
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

#[derive(Eq, PartialEq, Debug)]
pub struct Args {
    section: i32,
    level: i32,
    sound_enabled: bool,
}

fn parse_args() -> Result<Args, AnyError> {
    let raw_args = std::env::args().collect::<Vec<_>>();
    parse_args_pure(&raw_args)
}

fn parse_args_pure<S: AsRef<str>>(raw_args: &Vec<S>) -> Result<Args, AnyError> {
    let (positional, flags) = split_positional(raw_args);
    let section = parse_as(&positional, 1, STARTING_SECTION, "i32")?;
    let level = parse_as(&positional, 2, STARTING_LEVEL, "i32")?;
    let sound_enabled = flags.iter().find(|e| e.as_ref() == "--no-sound").is_none();

    Ok(Args {
        section,
        level,
        sound_enabled,
    })
}

fn split_positional<S: AsRef<str>>(raw_args: &Vec<S>) -> (Vec<&S>, Vec<&S>) {
    let mut positional = Vec::new();
    let mut flags = Vec::new();
    for arg in raw_args {
        if arg.as_ref().starts_with("--") {
            flags.push(arg);
        } else {
            positional.push(arg);
        }
    }
    (positional, flags)
}

fn parse_as<S: AsRef<str>>(
    raw_args: &[S],
    i: usize,
    default: i32,
    type_name: &str,
) -> Result<i32, AnyError> {
    if let Some(level_arg) = raw_args.get(i) {
        let level_arg = level_arg.as_ref();
        level_arg
            .parse()
            .map_err(|e| format!("error parsing '{}' as {}: {}", level_arg, type_name, e).into())
    } else {
        Ok(default)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args() {
        let input = vec!["metro-loop", "3", "4"];
        let parsed = parse_args_pure(&input).unwrap();
        assert_eq!(
            parsed,
            Args {
                section: 3,
                level: 4,
                sound_enabled: true
            }
        )
    }
    #[test]
    fn test_args_no_sound() {
        let input = vec!["metro-loop", "3", "--no-sound", "4"];
        let parsed = parse_args_pure(&input).unwrap();
        assert_eq!(
            parsed,
            Args {
                section: 3,
                level: 4,
                sound_enabled: false
            }
        )
    }
}
