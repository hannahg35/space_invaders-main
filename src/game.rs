use crate::custom::enemy::Enemy;
use crate::custom::laser::Bullet;
use crate::custom::lives::Lives;
use crate::custom::player::Player;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::collision::check_collision;
use crate::utils::preload_image::TextureManager;
use ::rand::Rng;

use macroquad::prelude::*;

fn spawn_enemy_wave(
    enemies: &mut Vec<Enemy>,
    tm: &TextureManager,
    columns: i32,
    rows: i32,
    spacing_x: f32,
    spacing_y: f32,
    start_x: f32,
    start_y: f32,
) {
    for row in 0..rows {
        for col in 0..columns {
            let enemy = Enemy::from_preload(
                tm.get_preload("assets/tiefighter.png").unwrap(),
                start_x + col as f32 * spacing_x,
                start_y + row as f32 * spacing_y,
                screen_width() * 0.07,
                screen_height() * 0.085,
                true,
                1.0,
            );
            enemies.push(enemy);
        }
    }
}

pub async fn run(
    tm: TextureManager,
    mut score: i32,
    mut highscore: i32,
    font: Font,
    yellow: Color,
    red: Color,
) -> (String, TextureManager, i32, i32, Font, Color, Color) {
    let mut player = Player::new(
        500.0,
        1045.0,
        screen_width() * 0.09,
        screen_height() * 0.1,
        "assets/xwing.png",
        true,
        1.0,
        vec2(35.0, 15.0),
    )
    .await;

    let img_background = StillImage::from_preload(
        tm.get_preload("assets/background.png").unwrap(),
        screen_width(),
        screen_height(),
        0.0,
        0.0,
        true,
        1.0,
    );

    let img_sides = StillImage::from_preload(
        tm.get_preload("assets/sides.png").unwrap(),
        screen_width(),
        screen_height(),
        0.0,
        0.0,
        true,
        1.0,
    );

    let mut rng = ::rand::rng();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemylasers: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut lives: Vec<Lives> = Vec::new();
    let mut shoot_cooldown = 0.0;
    let mut enemyshoot_cooldown = 2.0;
    let mut player_hit_cooldown = 0.0;
    let mut columns;
    let mut rows;
    let mut enemy_speed = 0.24;
    let mut wave = 1;
    let max_waves = 5;

    let mut lbl_score = Label::new(format!("Score:{}", score), 240.0, 60.0, 35);
    lbl_score.with_colors(BLACK, None);
    lbl_score.with_font(font.clone());

    let mut lbl_wave = Label::new(format!("Wave:{}", wave), 900.0, 60.0, 35);
    lbl_wave.with_colors(BLACK, None);
    lbl_wave.with_font(font.clone());

    let mut lbl_highscore = Label::new(format!("Highscore:{}", highscore), 1520.0, 60.0, 35);
    lbl_highscore.with_colors(BLACK, None);
    lbl_highscore.with_font(font.clone());

    let spacing_x = screen_width() * 0.06;
    let spacing_y = screen_height() * 0.09;
    let min_x = screen_width() * 0.18;
    let max_x = screen_width() * 0.82;
    let start_x = min_x;
    let enemy_start_y = 180.0;
    let fixed_columns: i32 = 8;
    let fixed_rows: i32 = 3;
    let mut player_hp = 3;

    for i in 0..player_hp {
        let life = Lives::from_preload(
            tm.get_preload("assets/lives.png").unwrap(),
            35.0 + i as f32 * 55.0,
            60.0,
            45.0,
            45.0,
            true,
            1.0,
        );
        lives.push(life);
    }

    columns = fixed_columns;
    rows = fixed_rows;
    spawn_enemy_wave(&mut enemies, &tm, columns, rows, spacing_x, spacing_y, start_x, enemy_start_y);

    loop {
        clear_background(WHITE);
        img_background.draw();
        player.get_image().draw();

        for enemy in &enemies {
            enemy.get_image().draw();
        }

        player.keypress();
        player.move_player();

        if player.move_dir.x != 0.0 && check_collision(player.get_image(), &img_sides, 1) {
            player.move_back_x();
        }

        let playershottime = get_frame_time();
        shoot_cooldown -= playershottime;

        if is_key_down(KeyCode::Space) && shoot_cooldown <= 0.0 {
            shoot_cooldown = 0.75;

            let bullet = Bullet::from_preload(
                tm.get_preload("assets/laser.png").unwrap(),
                player.image.get_x() + 25.0,
                player.image.get_y(),
                screen_width() * 0.012,
                screen_height() * 0.032,
                true,
                1.0,
                3.0,
            );

            bullets.push(bullet);
        }

        for bullet_index in (0..bullets.len()).rev() {
            let bullet = &mut bullets[bullet_index];
            bullet.playershoot();
            bullet.get_image().draw();

            if bullet.image.get_y() < -100.0 {
                bullet.image.clear();
                bullet.move_dir = vec2(0.0, 0.0);
                bullets.remove(bullet_index);
                continue;
            }

            for enemy_index in (0..enemies.len()).rev() {
                if check_collision(bullet.get_image(), enemies[enemy_index].get_image(), 1) {
                    enemies.remove(enemy_index);
                    bullets.remove(bullet_index);
                    score += 100;
                    lbl_score.set_text(format!("Score:{}", score));

                    if score > highscore {
                        highscore = score;
                    }

                    lbl_highscore.set_text(format!("Highscore:{}", highscore));
                    lbl_wave.set_text(format!("Wave:{}", wave));
                    break;
                }
            }
        }

        enemyshoot_cooldown -= playershottime;
        player_hit_cooldown = (player_hit_cooldown - playershottime).max(0.0);

        player_hit_cooldown -= playershottime;

        if player_hit_cooldown < 0.0 {
            player_hit_cooldown = 0.0;
        }

        let mut max_enemy_lasers = 0;
        if wave <= 1 {
            max_enemy_lasers = 0;
        } else {
            max_enemy_lasers = wave - 1;
        };

        if !enemies.is_empty() && enemylasers.len() < max_enemy_lasers && enemyshoot_cooldown <= 0.0 {
            enemyshoot_cooldown = (2.8 - wave as f32 * 0.14).clamp(1.4, 2.8);

            let enemy_index = rng.random_range(0..enemies.len());
            let randomenemy = &enemies[enemy_index];

            let enemybullet = Bullet::from_preload(
                tm.get_preload("assets/enemylaser.png").unwrap(),
                randomenemy.image.get_x() + 25.0,
                randomenemy.image.get_y(),
                screen_width() * 0.012,
                screen_height() * 0.032,
                true,
                1.0,
                3.0,
            );

            enemylasers.push(enemybullet);
        }

        for enemylaser_index in (0..enemylasers.len()).rev() {
            let enemybullet = &mut enemylasers[enemylaser_index];
            enemybullet.enemyshoot();
            enemybullet.get_image().draw();

            if enemybullet.image.get_y() > screen_height() + 100.0 {
                enemybullet.image.clear();
                enemybullet.move_dir = vec2(0.0, 0.0);
                enemylasers.remove(enemylaser_index);
                continue;
            }

            if player_hit_cooldown <= 0.0 && check_collision(enemybullet.get_image(), player.get_image(), 1) {
                enemylasers.remove(enemylaser_index);

                if !lives.is_empty() {
                    lives.pop();
                }

                player_hp -= 1;
                player_hit_cooldown = 1.1;

                if player_hp <= 0 {
                    return ("lose".to_string(), tm, score, highscore, font, yellow, red);
                }

                break;
            }
        }

        if enemies.is_empty() {
            if wave >= max_waves {
                return ("win".to_string(), tm, score, highscore, font, yellow, red);
            }

            wave += 1;
            lbl_wave.set_text(format!("Wave:{}", wave));
            if wave > 1 {
                enemy_speed = 0.18;
            }
            columns = fixed_columns;
            rows = fixed_rows;
            spawn_enemy_wave(&mut enemies, &tm, columns, rows, spacing_x, spacing_y, min_x, enemy_start_y);

            if player_hp < 3 {
                player_hp += 1;
                let life = Lives::from_preload(
                    tm.get_preload("assets/lives.png").unwrap(),
                    35.0 + lives.len() as f32 * 55.0,
                    60.0,
                    45.0,
                    45.0,
                    true,
                    1.0,
                );
                lives.push(life);
            }

            enemyshoot_cooldown = 1.0;
            enemylasers.clear();
        }

        for enemy in &mut enemies {
            enemy.image.set_x(enemy.image.get_x() + enemy_speed * 3.2);
        }

        let mut leftside = screen_width();
        let mut rightside: f32 = 0.0;

        for enemy in &enemies {
            let x = enemy.image.get_x();

            leftside = leftside.min(x);
            rightside = rightside.max(x);
        }

        if rightside >= max_x || leftside <= min_x {
            for enemy in &mut enemies {
                enemy.image.set_x(enemy.image.get_x() - enemy_speed * 3.2);
                enemy.image.set_y(enemy.image.get_y() + 28.0);
            }
            enemy_speed *= -1.0;
        }

        for enemy in &enemies {
            let img = enemy.get_image();

            if img.get_y() + img.size().y >= 1050.0 {
                return ("lose".to_string(), tm, score, highscore, font, yellow, red);
            }
        }

        img_sides.draw();
        lbl_score.draw();
        lbl_highscore.draw();
        lbl_wave.draw();

        for life in &lives {
            life.get_image().draw();
        }

        next_frame().await;
    }
}
