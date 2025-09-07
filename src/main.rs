use macroquad::miniquad::date::now;
use macroquad::prelude::{next_frame, Conf};
use macroquad::rand::srand;
use metro_loop::{
    scenes, AnyError, NextStage, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE, DEFAULT_WINDOW_WIDTH,
    STARTING_LEVEL, STARTING_SECTION,
};

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let seed = now() as u64;
    srand(seed);

    let args = parse_args()?;
    let mut theme = scenes::loading_screen(args.section, args.level).await?;
    let mut next_stage = NextStage::MainMenu;
    loop {
        next_stage = match next_stage {
            NextStage::MainMenu => scenes::main_menu(&mut theme).await?,
            NextStage::Campaign => scenes::play(&mut theme).await?,
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

pub struct Args {
    section: i32,
    level: i32,
}

fn parse_args() -> Result<Args, AnyError> {
    let raw_args = std::env::args().collect::<Vec<_>>();
    let section = parse_as(&raw_args, 1, STARTING_SECTION, "i32")?;
    let level = parse_as(&raw_args, 2, STARTING_LEVEL, "i32")?;
    Ok(Args { section, level })
}

fn parse_as(raw_args: &[String], i: usize, default: i32, type_name: &str) -> Result<i32, AnyError> {
    if let Some(level_arg) = raw_args.get(i) {
        level_arg
            .parse()
            .map_err(|e| format!("error parsing '{}' as {}: {}", level_arg, type_name, e).into())
    } else {
        Ok(default)
    }
}
