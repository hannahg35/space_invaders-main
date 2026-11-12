use crate::ui::still_image::StillImage;
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct Player {
    pub image: StillImage,
    pub speed: f32,
    pub move_dir: Vec2,
    pub old_pos: Vec2,
}
impl Player {
    pub async fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        image_name: &str,
        stretch: bool,
        zoom: f32,
        old_pos: Vec2,
    ) -> Self {
        let image = StillImage::new(image_name, width, height, x, y, stretch, zoom).await;

        Player {
            image,
            speed: 1.8,
            move_dir: vec2(0.0, 0.0),
            old_pos,
        }
    }

    pub fn get_image(&self) -> &StillImage {
        &self.image
    }

    pub fn keypress(&mut self) {
  
            // Direction to move in
            self.move_dir = vec2(0.0, 0.0);
            self.old_pos = vec2(self.image.get_x(), self.image.get_y());

            // Keyboard input
            if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                self.move_dir.x += self.speed;
            }

            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                self.move_dir.x -= self.speed;
            }

            if self.move_dir.length() > 0.0 {
                self.move_dir = self.move_dir.normalize();
            }
    
    }

    pub fn move_player(&mut self) {
        self.image.set_x(self.image.get_x() + self.move_dir.x);
    }

    pub fn move_back_x(&mut self) {
        self.image.set_x(self.old_pos.x)
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

}