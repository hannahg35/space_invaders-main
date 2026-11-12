use crate::ui::still_image::StillImage;
use macroquad::prelude::*;

pub struct Enemy {
    pub image: StillImage,
    pub speed: f32,
    pub move_dir: Vec2,
}
impl Enemy {
    pub async fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        image_name: &str,
        stretch: bool,
        zoom: f32,
    ) -> Self {
        let image = StillImage::new(image_name, width, height, x, y, stretch, zoom).await;

        Enemy {
            image,
            speed: 1.0,
            move_dir: vec2(0.0, 0.0),
        }
    }

    pub fn from_preload(
        preloaded: (Texture2D, Option<Vec<u8>>, String),
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        stretch: bool,
        zoom: f32,
    ) -> Self {
        let image = StillImage::from_preload(preloaded, width, height, x, y, stretch, zoom);

        Enemy {
            image,
            speed: 1.0,
            move_dir: vec2(0.0, 0.0),
        }
    }


    pub fn get_image(&self) -> &StillImage {
        &self.image
    }
   
}
