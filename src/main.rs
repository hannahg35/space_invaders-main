/*
By: By: <Hannah Ursula Greig>
Date: 2026-10-21
Program Details: <A reacreation of Space Invaders, but with a Star Wars Theme.>
*/

mod custom;
mod game;
mod lose;
mod title;
mod tutorial;
mod ui;
mod utils;
mod win;
use crate::utils::preload_image::TextureManager;

use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "spaceinvaders".to_string(),
        window_width: 2000,
        window_height: 1250,
        fullscreen: false,
        high_dpi: true,
        window_resizable: false,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut tm = TextureManager::new();

    tm.preload_with_loading_screen(
        &[
            "assets/title.png",
            "assets/xwing.png",
            "assets/tiefighter.png",
            "assets/laser.png",
            "assets/enemylaser.png",
            "assets/background.png",
            "assets/sides.png",
            "assets/lose.png",
            "assets/win.png",
            "assets/tutorial.png",
            "assets/lives.png",
            "assets/play.png",
            "assets/playpress.png",
            "assets/tut.png",
            "assets/tutpress.png",
            "assets/exit.png",
            "assets/exitpress.png",
            "assets/back.png",
            "assets/backpress.png",
        ],
        None,
        None,
    )
    .await;

    let mut score = 0;
    let mut highscore = 0;

    let mut current_screen = "title".to_string();
    let mut last_switch = get_time() - 0.02;

    let mut yellow = Color::from_hex(0xf2cc5b);
    let mut red = Color::from_hex(0xc75353);
    let mut font = load_ttf_font("assets/astro.ttf").await.unwrap();

    loop {
        if get_time() - last_switch > 0.01 {
            (current_screen, tm, score, highscore, font, yellow, red) = match current_screen.as_str() {
                "title" => title::run(tm, score, highscore, font, yellow, red).await,
                "game" => game::run(tm, score, highscore, font, yellow, red).await,
                "win" => win::run(tm, score, highscore, font, yellow, red).await,
                "lose" => lose::run(tm, score, highscore, font, yellow, red).await,
                "tutorial" => tutorial::run(tm, score, highscore, font, yellow, red).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}
