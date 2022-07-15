use super::Texture;
use crate::utility::*;

use image::{ImageBuffer, RgbImage};
use rand::Rng;

#[derive(Clone)]
pub struct ImageTexture {
    img: RgbImage,
}

impl ImageTexture {
    pub fn new_form_file(filename: &str) -> Self {
        Self {
            img: match image::open(filename) {
                Ok(img) => img,
                Err(_) => panic!("Couldn't open file: {}", filename),
            }
            .into_rgb8(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // image库坐标y相反

        let i = ((u * self.img.width() as f64) as u32).clamp(0, self.img.width() - 1);
        let j = ((v * self.img.height() as f64) as u32).clamp(0, self.img.height() - 1);

        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i, j);
        Color::new(
            pixel[0] as f64 * color_scale,
            pixel[1] as f64 * color_scale,
            pixel[2] as f64 * color_scale,
        )
    }
}

#[derive(Clone)]
pub struct RandImageTexture {
    img: RgbImage,
}

impl RandImageTexture {
    pub fn new_form_file(filename: &str) -> Self {
        Self {
            img: match image::open(filename) {
                Ok(img) => img,
                Err(_) => panic!("Couldn't open file: {}", filename),
            }
            .into_rgb8(),
        }
    }
}

impl Texture for RandImageTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..self.img.width());
        let j = rng.gen_range(0..self.img.height());

        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i, j);
        Color::new(
            pixel[0] as f64 * color_scale,
            pixel[1] as f64 * color_scale,
            pixel[2] as f64 * color_scale,
        )
    }
}
