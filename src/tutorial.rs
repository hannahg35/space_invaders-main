use crate::ui::grid::draw_grid;
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
    let img_tut = StillImage::from_preload(
        tm.get_preload("assets/tutorial.png").unwrap(),
        screen_width(),
        screen_height(),
        0.0,
        0.0,
        true,
        1.0,
    );

       let btn_back = ImageButton::from_preload(
        775.0,
        1050.0,
        350.0,
        150.0,
        tm.get_preload("assets/back.png").unwrap(),
        tm.get_preload("assets/backpress.png").unwrap(),
    );

    let mut lbl_tut = Label::new(
        format!(
            "Use A and D to move around.\n
Press Space to fire lasers at enemy ships.\n
Defend off the hoards of tiefighters.\n
Don't let them reach you, good luck Rebel!"
        ),
        200.0,
        500.0,
        60,
    );
    lbl_tut.with_colors(yellow, None);
    lbl_tut.with_font(font.clone());
    lbl_tut.with_alignment(crate::ui::label::TextAlign::Center);

    loop {
        clear_background(WHITE);
        img_tut.draw();
        lbl_tut.draw();
       // draw_grid(50.0, BROWN);

        if is_key_pressed(KeyCode::Space) {
            return ("title".to_string(), tm, score, highscore, font, yellow, red);
        }

        if btn_back.click() {
            return ("title".to_string(), tm, score, highscore, font, yellow, red);
        }

        next_frame().await;
    }
}
