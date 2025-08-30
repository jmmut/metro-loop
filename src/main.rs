use macroquad::miniquad::date::now;
use macroquad::prelude::Conf;
use macroquad::rand::srand;
use metro_loop::{
    scenes, AnyError, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE, DEFAULT_WINDOW_WIDTH,
};

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let seed = now() as u64;
    srand(seed);

    scenes::loading_screen().await?;
    scenes::main_menu().await?;
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
