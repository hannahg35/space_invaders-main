use crate::ui::image_button::ImageButton;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

use macroquad::prelude::*;

pub async fn run(
    tm: TextureManager,
    mut score: i32,
    mut highscore: i32,
    font: Font,
    yellow: Color,
    red: Color,
) -> (String, TextureManager, i32, i32, Font, Color, Color) {
    let img_win = StillImage::from_preload(
        tm.get_preload("assets/win.png").unwrap(),
        screen_width(),
        screen_height(),
        0.0,
        0.0,
        true,
        1.0,
    );

    let btn_back = ImageButton::from_preload(
        775.0,
        850.0,
        350.0,
        150.0,
        tm.get_preload("assets/back.png").unwrap(),
        tm.get_preload("assets/backpress.png").unwrap(),
    );

    let mut lbl_score = Label::new(format!("Score:{}", score), 700.0, 700.0, 60);
    lbl_score.with_colors(yellow, None);
    lbl_score.with_font(font.clone());

    loop {
        clear_background(WHITE);
        img_win.draw();
        lbl_score.draw();
        if btn_back.click() {
            return ("title".to_string(), tm, score, highscore, font, yellow, red);
        }

        next_frame().await;
    }
}
