use crate::ui::image_button::ImageButton;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

use macroquad::prelude::*;

pub async fn run(
    tm: TextureManager,
    score: i32,
    highscore: i32,
    font: Font,
    yellow: Color,
    red: Color,
) -> (String, TextureManager, i32, i32, Font, Color, Color) {
    let title = StillImage::from_preload(
        tm.get_preload("assets/title.png").unwrap(),
        screen_width(),
        screen_height(),
        0.0,
        0.0,
        true,
        1.0,
    );

    let mut lbl_highscore = Label::new(format!("Highscore:{}", highscore), 700.0, 610.0, 60);
    lbl_highscore.with_colors(yellow, None);
    lbl_highscore.with_font(font.clone());

    let btn_exit = ImageButton::from_preload(
        775.0,
        1050.0,
        350.0,
        150.0,
        tm.get_preload("assets/exit.png").unwrap(),
        tm.get_preload("assets/exitpress.png").unwrap(),
    );

    let btn_tutorial: ImageButton = ImageButton::from_preload(
        775.0,
        850.0,
        350.0,
        150.0,
        tm.get_preload("assets/tut.png").unwrap(),
        tm.get_preload("assets/tutpress.png").unwrap(),
    );

    let btn_play = ImageButton::from_preload(
        775.0,
        650.0,
        350.0,
        150.0,
        tm.get_preload("assets/play.png").unwrap(),
        tm.get_preload("assets/playpress.png").unwrap(),
    );

    loop {
        clear_background(WHITE);
        title.draw();
        lbl_highscore.draw();

        if btn_play.click() {
            return ("game".to_string(), tm, score, highscore, font, yellow, red);
        }

        if btn_tutorial.click() {
            return ("tutorial".to_string(), tm, score, highscore, font, yellow, red);
        }

        if btn_exit.click() {
            return ("exit".to_string(), tm, score, highscore, font, yellow, red);
        }

        next_frame().await;
    }
}
