use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::text::TextRect;
use macroquad::color::{BLACK, GRAY};
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::math::Rect;
use macroquad::prelude::{clear_background, next_frame, Conf};
use metro_loop::slider::Slider;
use metro_loop::{
    AnyError, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE, DEFAULT_WINDOW_WIDTH, STYLE,
};

#[macroquad::main(window_conf)]
async fn main() -> Result<(), AnyError> {
    let mut current = 0.5;
    let rect = Rect::new(50.0, 20.0, 50.0, 15.0);
    let mut slider = Slider::new(0.0, 1.0, current, rect);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(GRAY);
        current = slider.interact();
        slider.render(&STYLE);

        let anchor = Anchor::below(rect, Horizontal::Left, 10.0);
        TextRect::new(&format!("value: {}", current), anchor, 16.0).render_text(BLACK);

        next_frame().await;
    }
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
